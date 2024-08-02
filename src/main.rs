mod app;
mod masses;
mod user_space_manager;

use masses::*;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<app::App>::new().render();
}
