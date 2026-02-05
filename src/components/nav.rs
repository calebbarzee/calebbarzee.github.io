use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="site-nav">
            <div class="container">
                <span class="prompt">"visitor@portfolio:~$ "</span>
                <a href="#about">"about"</a>
                <span class="separator">" | "</span>
                <a href="#projects">"projects"</a>
                <span class="separator">" | "</span>
                <a href="#contact">"contact"</a>
            </div>
        </nav>
    }
}
