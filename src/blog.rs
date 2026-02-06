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

// ── Blog Registry ──────────────────────────────────────────
// Add new posts here by including the markdown file.

const POST_CALCULATING_PI: &str = include_str!("posts/pi_calc_blog1.md");
const POST_GENERATIVE_AI: &str = include_str!("posts/gen_ai_blog1.md");
const POST_RETHINKING_RESUME: &str = include_str!("posts/rethinking_the_resume.md");

/// Get all blog posts, sorted by date (newest first).
pub fn get_all_posts() -> Vec<BlogPost> {
    let mut posts = vec![
        BlogPost::from_markdown("calculating-pi", POST_CALCULATING_PI),
        BlogPost::from_markdown("generative-ai", POST_GENERATIVE_AI),
        BlogPost::from_markdown("rethinking-the-resume", POST_RETHINKING_RESUME),
    ];

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
        assert_eq!(posts.len(), 3);
        // Check they're sorted by date descending
        assert!(posts[0].date >= posts[1].date);
    }
}
