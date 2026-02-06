use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::*;

/// Main portfolio page (home)
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Header />
        <Nav />
        <Hero />
        <About />
        <Projects />
        <Footer />
    }
}

/// Gallery grid showing all scene+effect combinations
#[component]
fn GalleryPage() -> impl IntoView {
    view! {
        <Header />
        <Nav />
        <gallery::Gallery />
        <Footer />
    }
}

/// Gallery viewer for a specific scene+effect combination
#[component]
fn GalleryViewerPage() -> impl IntoView {
    view! {
        <gallery::GalleryViewer />
    }
}

/// Blog listing page
#[component]
fn BlogPage() -> impl IntoView {
    view! {
        <Header />
        <Nav />
        <blog::BlogList />
        <Footer />
    }
}

/// Individual blog post page
#[component]
fn BlogPostPage() -> impl IntoView {
    view! {
        <Header />
        <Nav />
        <blog::BlogPostView />
        <Footer />
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"404 - Page not found"</p> }>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/gallery") view=GalleryPage />
                <Route path=path!("/gallery/:scene/:effect") view=GalleryViewerPage />
                <Route path=path!("/blog") view=BlogPage />
                <Route path=path!("/blog/:slug") view=BlogPostPage />
            </Routes>
        </Router>
    }
}
