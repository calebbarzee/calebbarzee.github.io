use pulldown_cmark::{html, Options, Parser};

/// Metadata for a blog post, parsed from frontmatter.
#[derive(Clone, Debug)]
pub struct BlogPost {
    pub slug: &'static str,
    pub title: String,
    pub subtitle: Option<String>,
    pub date: String,
    pub content_html: String,
}

impl BlogPost {
    /// Create a BlogPost from raw markdown content with frontmatter.
    pub fn from_markdown(slug: &'static str, raw: &str) -> Self {
        let (frontmatter, content) = parse_frontmatter(raw);

        let title = frontmatter
            .iter()
            .find(|(k, _)| *k == "title")
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| slug.to_string());

        let subtitle = frontmatter
            .iter()
            .find(|(k, _)| *k == "subtitle")
            .map(|(_, v)| v.clone());

        let date = frontmatter
            .iter()
            .find(|(k, _)| *k == "date_written" || *k == "date")
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let content_html = markdown_to_html(content);

        Self {
            slug,
            title,
            subtitle,
            date,
            content_html,
        }
    }
}

/// Parse simple YAML-like frontmatter from markdown.
/// Returns (key-value pairs, remaining content).
fn parse_frontmatter(raw: &str) -> (Vec<(&str, String)>, &str) {
    let raw = raw.trim();

    if !raw.starts_with("---") {
        return (vec![], raw);
    }

    // Find the closing ---
    let rest = &raw[3..];
    if let Some(end_idx) = rest.find("\n---") {
        let frontmatter_str = &rest[..end_idx];
        let content = &rest[end_idx + 4..].trim_start();

        let pairs: Vec<(&str, String)> = frontmatter_str
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }
                let mut parts = line.splitn(2, ':');
                let key = parts.next()?.trim();
                let value = parts.next()?.trim();
                // Remove surrounding quotes if present
                let value = value
                    .trim_start_matches('"')
                    .trim_end_matches('"')
                    .to_string();
                Some((key, value))
            })
            .collect();

        (pairs, content)
    } else {
        (vec![], raw)
    }
}

/// Convert markdown to HTML using pulldown-cmark.
fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

// ── Blog Registry (auto-generated from src/articles/*.md) ──

include!(concat!(env!("OUT_DIR"), "/articles.rs"));

/// Get all blog posts, sorted by date (newest first).
pub fn get_all_posts() -> Vec<BlogPost> {
    let mut posts: Vec<BlogPost> = ARTICLES
        .iter()
        .map(|(slug, content)| BlogPost::from_markdown(slug, content))
        .collect();

    // Sort by date descending (newest first)
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    posts
}

/// Get a single blog post by slug.
pub fn get_post_by_slug(slug: &str) -> Option<BlogPost> {
    get_all_posts().into_iter().find(|p| p.slug == slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let raw = r#"---
title: "Test Post"
date_written: "2024-01-15"
---

# Content here
"#;
        let (fm, content) = parse_frontmatter(raw);
        assert_eq!(fm.len(), 2);
        assert_eq!(fm[0], ("title", "Test Post".to_string()));
        assert_eq!(fm[1], ("date_written", "2024-01-15".to_string()));
        assert!(content.starts_with("# Content"));
    }

    #[test]
    fn test_markdown_to_html() {
        let md = "# Hello\n\nThis is **bold**.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_get_all_posts() {
        let posts = get_all_posts();
        assert!(!posts.is_empty(), "Should have at least one blog post");
        // Check they're sorted by date descending
        for w in posts.windows(2) {
            assert!(w[0].date >= w[1].date, "Posts should be sorted by date descending");
        }
    }
}
