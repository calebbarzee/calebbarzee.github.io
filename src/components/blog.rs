use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::blog::{get_all_posts, get_post_by_slug};

/// Blog listing page showing all posts.
#[component]
pub fn BlogList() -> impl IntoView {
    let posts = get_all_posts();

    view! {
        <section class="blog-list">
            <div class="container">
                <h2>"$ ls blog/"</h2>
                <div class="blog-posts">
                    {posts.into_iter().map(|post| {
                        let href = format!("/blog/{}", post.slug);
                        view! {
                            <A href=href attr:class="blog-post-card">
                                <article>
                                    <h3>{post.title}</h3>
                                    {post.subtitle.map(|s| view! { <p class="subtitle">{s}</p> })}
                                    <time>{post.date}</time>
                                </article>
                            </A>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

/// Individual blog post page.
#[component]
pub fn BlogPostView() -> impl IntoView {
    let params = use_params_map();

    let post = move || {
        let p = params.read();
        let slug = p.get("slug").unwrap_or_default();
        get_post_by_slug(&slug)
    };

    view! {
        <div class="blog-post-page">
            {move || {
                if let Some(p) = post() {
                    view! {
                        <article class="blog-post">
                            <header class="blog-post-header">
                                <A href="/blog" attr:class="back-link">"< back to blog"</A>
                                <h1>{p.title.clone()}</h1>
                                {p.subtitle.clone().map(|s| view! { <p class="subtitle">{s}</p> })}
                                <time>{p.date.clone()}</time>
                            </header>
                            <div class="blog-post-content" inner_html=p.content_html.clone()></div>
                        </article>
                    }.into_any()
                } else {
                    view! {
                        <div class="blog-post-error">
                            <p>"Post not found"</p>
                            <A href="/blog">"Back to blog"</A>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
