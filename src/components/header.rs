use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="container">
                <pre class="ascii-logo">{r#"
  ____      _      _      ____
 / ___|__ _| | ___| |__  | __ )
| |   / _` | |/ _ \ '_ \ |  _ \
| |__| (_| | |  __/ |_) || |_) |
 \____\__,_|_|\___|_.__/ |____/
"#}</pre>
            </div>
        </header>
    }
}
