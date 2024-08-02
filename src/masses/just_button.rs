use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
    let usr_mmr_ref = use_node_ref();
    let usr_1_mmr_ref = use_node_ref();
    let usr_2_mmr_ref = use_node_ref();
    let usr_3_mmr_ref = use_node_ref();
    let usr_4_mmr_ref = use_node_ref();
    let usr_5_mmr_ref = use_node_ref();

    let usr_mmr_basic = use_state(|| 50.0);

    let usr_name_1 = use_state(|| String::new());
    let usr_name_2 = use_state(|| String::new());
    let usr_name_3 = use_state(|| String::new());
    let usr_name_4 = use_state(|| String::new());
    let usr_name_5 = use_state(|| String::new());

    let show_box_1 = use_state(|| false);
    let show_box_2 = use_state(|| false);

    // Callback for handling button presses
    let handle_button_press = |usr_name: UseStateHandle<String>, show_box: UseStateHandle<bool>, input_ref: NodeRef| {
        Callback::from(move |_: MouseEvent| {
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            if *show_box {
                show_box.set(false);
                usr_name.set(String::new());
            } else {
                usr_name.set(input.value());
                show_box.set(true);
            }
            input.set_value("");
        })
    };

    let usr_1 = handle_button_press(usr_name_1.clone(), show_box_1.clone(), usr_1_mmr_ref.clone());
    let usr_2 = handle_button_press(usr_name_2.clone(), show_box_2.clone(), usr_2_mmr_ref.clone());
    // Repeat for usr_3, usr_4, and usr_5 if needed

    html! {
        <main class="container">
            <div class="row">
                <input type="text" id="usr-1-input" ref={usr_1_mmr_ref} />
                <button onclick={usr_1}>{"Toggle User 1"}</button>
                if *show_box_1 {
                    <div class="user-box">
                        <h3>{"User 1"}</h3>
                        <p>{format!("Name: {}", *usr_name_1)}</p>
                    </div>
                }
            </div>
            <div class="row">
                <input type="text" id="usr-2-input" ref={usr_2_mmr_ref} />
                <button onclick={usr_2}>{"Toggle User 2"}</button>
                if *show_box_2 {
                    <div class="user-box">
                        <h3>{"User 2"}</h3>
                        <p>{format!("Name: {}", *usr_name_2)}</p>
                    </div>
                }
            </div>
        </main>
    }
}
