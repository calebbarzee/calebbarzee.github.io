use leptos::prelude::*;
use crate::components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <Nav />
        <Hero />
        <About />
        <Projects />
        <Footer />
    }
}
