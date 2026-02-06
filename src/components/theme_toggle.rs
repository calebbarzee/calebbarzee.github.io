use leptos::prelude::*;

fn get_document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn current_theme() -> String {
    get_document()
        .document_element()
        .and_then(|el| el.get_attribute("data-theme"))
        .unwrap_or_else(|| "dark".to_string())
}

fn set_theme(theme: &str) {
    let doc = get_document();
    if let Some(el) = doc.document_element() {
        let _ = el.set_attribute("data-theme", theme);
    }
    if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
        let _ = storage.set_item("theme", theme);
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (is_dark, set_is_dark) = signal(current_theme() == "dark");

    let toggle = move |_: web_sys::MouseEvent| {
        let new_dark = !is_dark.get_untracked();
        let theme = if new_dark { "dark" } else { "light" };
        set_theme(theme);
        set_is_dark.set(new_dark);
    };

    let label = move || {
        if is_dark.get() {
            "[light]"
        } else {
            "[dark]"
        }
    };

    view! {
        <button class="theme-toggle" on:click=toggle>
            {label}
        </button>
    }
}
