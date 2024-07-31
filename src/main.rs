mod app;
mod masses;

use masses::*;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<test2::App>::new().render();
}
