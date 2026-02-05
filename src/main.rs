mod app;
mod components;
mod webgl;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("failed to init logger");
    leptos::mount::mount_to_body(App);
}
