use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="about">
            <div class="container">
                <h2>"$ cat about.txt"</h2>
                <div class="bio">
                    <p>
                        "Hi, I'm Caleb — a software engineer who loves building performant, "
                        "creative web experiences. I work across the stack with Rust, TypeScript, "
                        "and whatever tools get the job done right."
                    </p>
                    <p>
                        "When I'm not writing code, you'll find me exploring new frameworks, "
                        "tinkering with graphics programming, or optimizing things that "
                        "probably don't need optimizing."
                    </p>
                </div>
            </div>
        </section>
    }
}
