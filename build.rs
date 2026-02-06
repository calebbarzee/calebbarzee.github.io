use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn main() {
    generate_theme_scss();
    generate_articles_registry();
}

fn generate_theme_scss() {
    println!("cargo:rerun-if-changed=style/theme/color_scheme.toml");

    let toml_str = fs::read_to_string("style/theme/color_scheme.toml")
        .expect("Failed to read color_scheme.toml");

    // TOML doesn't support // comments, so strip them
    let toml_str: String = toml_str
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    let parsed: toml::Value = toml::from_str(&toml_str).expect("Failed to parse color_scheme.toml");

    let mut scss = String::new();

    for theme in &["dark", "light"] {
        let table = parsed
            .get(theme)
            .and_then(|v| v.as_table())
            .unwrap_or_else(|| panic!("Missing [{theme}] table in color_scheme.toml"));

        // BTreeMap for deterministic order
        let sorted: BTreeMap<&String, &toml::Value> = table.iter().collect();

        scss.push_str(&format!("[data-theme=\"{theme}\"] {{\n"));
        for (key, value) in &sorted {
            let css_var = key.replace('_', "-");
            let val = value.as_str().unwrap_or_else(|| {
                panic!("Expected string value for key '{key}' in [{theme}]")
            });
            scss.push_str(&format!("    --{css_var}: {val};\n"));
        }
        scss.push_str("}\n\n");
    }

    let out_path = Path::new("style/theme/_generated.scss");
    fs::write(out_path, &scss).expect("Failed to write _generated.scss");
}

fn generate_articles_registry() {
    println!("cargo:rerun-if-changed=src/articles");

    let articles_dir = Path::new("src/articles");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = Path::new(&out_dir).join("articles.rs");

    let mut entries: Vec<(String, String)> = Vec::new();

    if articles_dir.is_dir() {
        let mut files: Vec<_> = fs::read_dir(articles_dir)
            .expect("Failed to read src/articles/")
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "md")
                    .unwrap_or(false)
            })
            .collect();

        files.sort_by_key(|e| e.file_name());

        for entry in files {
            let path = entry.path();
            let stem = path
                .file_stem()
                .expect("Missing file stem")
                .to_string_lossy()
                .to_string();

            let slug = stem.replace('_', "-");
            let abs_path = fs::canonicalize(&path)
                .expect("Failed to canonicalize article path")
                .display()
                .to_string();

            entries.push((slug, abs_path));
        }
    }

    let mut code = String::from("pub const ARTICLES: &[(&str, &str)] = &[\n");
    for (slug, abs_path) in &entries {
        code.push_str(&format!(
            "    (\"{slug}\", include_str!(\"{abs_path}\")),\n"
        ));
    }
    code.push_str("];\n");

    fs::write(&out_path, &code).expect("Failed to write articles.rs");
}
