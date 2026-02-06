use leptos::prelude::*;
use leptos_router::components::A;

use super::ThemeToggle;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="site-nav">
            <div class="container">
                <span class="prompt">"visitor@portfolio:~$ "</span>
                <A href="/">"home"</A>
                <span class="separator">" | "</span>
                <A href="/gallery">"gallery"</A>
                <span class="separator">" | "</span>
                <A href="/viewer/stl">"3d-viewer"</A>
                <span class="separator">" | "</span>
                <A href="/viewer/image">"image-fx"</A>
                <span class="separator">" | "</span>
                <A href="/blog">"blog"</A>
                <div class="nav-spacer"></div>
                <ThemeToggle />
            </div>
        </nav>
    }
}
