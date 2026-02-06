use leptos::prelude::*;
use leptos_router::components::A;

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
                <A href="/blog">"blog"</A>
                <span class="separator">" | "</span>
                <a href="#about">"about"</a>
                <span class="separator">" | "</span>
                <a href="#projects">"projects"</a>
                <span class="separator">" | "</span>
                <a href="#contact">"contact"</a>
            </div>
        </nav>
    }
}
