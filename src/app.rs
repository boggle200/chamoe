use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::user_space_manager::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let guest_1_ref = use_node_ref();
    let guest_1_name = use_state(|| String::new());
    let guest_1_permission = use_state(|| false);
    let guest_1_state_box = use_state(|| false);

    let handle_guest_1_box = {
        let guest_1_state_box = guest_1_state_box.clone();
        Callback::from(move |_| {
            guest_1_state_box.set(!*guest_1_state_box);
        })
    };

    let guest_2_ref = use_node_ref();
    let guest_2_name = use_state(|| String::new());
    let guest_2_permission = use_state(|| false);
    let guest_2_state_box = use_state(|| false);

    let handle_guest_2_box = {
        let guest_2_state_box = guest_2_state_box.clone();
        Callback::from(move |_| {
            guest_2_state_box.set(!*guest_2_state_box);
        })
    };

    let guest_3_ref = use_node_ref();
    let guest_3_name = use_state(|| String::new());
    let guest_3_permission = use_state(|| false);
    let guest_3_state_box = use_state(|| false);

    let handle_guest_3_box = {
        let guest_3_state_box = guest_3_state_box.clone();
        Callback::from(move |_| {
            guest_3_state_box.set(!*guest_3_state_box);
        })
    };
    html! {
        <main class="container">
            <p>{"It's time to start your cloud on your computer!"}</p>
            
            <div class="row">
                <button onclick={handle_guest_1_box.clone()}>{"Guest 1"}</button>
                if *guest_1_state_box {
                    <div class="guest-box">
                        <h3>{"User 1"}</h3>
                        <p>{"Hello World!"}</p>
                    </div>
                }
            </div>
            <div class="row">
                <button onclick={handle_guest_2_box.clone()}>{"Guest 2"}</button>
                if *guest_2_state_box {
                    <div class="guest-box">
                        <h3>{"User 2"}</h3>
                        <p>{"Hello World!"}</p>
                    </div>
                }
            </div>
            <div class="row">
                <button onclick={handle_guest_3_box.clone()}>{"Guest 3"}</button>
                if *guest_3_state_box {
                    <div class="guest-box">
                        <h3>{"User 3"}</h3>
                        <p>{"Hello World!"}</p>
                    </div>
                }
            </div>
        </main>
    }
}
