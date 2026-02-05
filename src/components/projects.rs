use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="projects">
            <div class="container">
                <h2>"$ ls projects/"</h2>
                <div class="project-grid">
                    <ProjectCard
                        name="ascii_render"
                        desc="This very site — a Rust/WASM portfolio with GPU-driven ASCII art rendering."
                        tech="Rust, Leptos, WebGL2, GLSL"
                    />
                    <ProjectCard
                        name="project_alpha"
                        desc="A high-performance data pipeline for real-time analytics."
                        tech="Rust, Kafka, PostgreSQL"
                    />
                    <ProjectCard
                        name="project_beta"
                        desc="Full-stack web app with server-side rendering and edge deployment."
                        tech="TypeScript, Next.js, Vercel"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(name: &'static str, desc: &'static str, tech: &'static str) -> impl IntoView {
    view! {
        <div class="project-card">
            <h3>{format!("./{}", name)}</h3>
            <p class="project-desc">{desc}</p>
            <span class="project-tech">{tech}</span>
        </div>
    }
}
