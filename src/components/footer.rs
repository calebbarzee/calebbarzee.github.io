use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer id="contact" class="site-footer">
            <div class="container">
                <p>"visitor@portfolio:~$ echo $CONTACT"</p>
                <div class="footer-links">
                    <a href="https://github.com/calebbarzee" target="_blank">"[GitHub]"</a>
                    <a href="https://linkedin.com/in/calebbarzee" target="_blank">"[LinkedIn]"</a>
                </div>
                <p class="copyright">"© 2025 Caleb Barzee. Built with Rust + WebAssembly."</p>
            </div>
        </footer>
    }
}
