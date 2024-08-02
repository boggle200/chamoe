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

    let usr_1 = {
        let usr_name_1 = usr_name_1.clone();
        let usr_1_mmr_ref = usr_1_mmr_ref.clone();
        let show_box_1 = show_box_1.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input_1 = usr_1_mmr_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            if *show_box_1 {
                show_box_1.set(false);
                usr_name_1.set(String::new());
            } else {
                usr_name_1.set(input_1.value());
                show_box_1.set(true);
            }
            input_1.set_value("");
        })
    };
    let usr_2 = {
        let usr_name_2 = usr_name_2.clone();
        let usr_2_mmr_ref = usr_2_mmr_ref.clone();
        let show_box_2 = show_box_2.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input_2 = usr_2_mmr_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            usr_name_2.set(input_2.value());
            show_box_2.set(true);
            input_2.set_value("");
        })
    };
    let usr_3 = {
        let usr_name_3 = usr_name_3.clone();
    };
    let usr_4 = {
        let usr_name_4 = usr_name_4.clone();
    };
    let usr_5 = {
        let usr_name_5 = usr_name_5.clone();
    };


    html! {
        <main class="container">
            <form class="row" onsubmit={usr_1}>
                <input id="usr-1-input" ref={usr_1_mmr_ref} />
                <button type="submit">{"User 1"}</button>
                if *show_box_1 {
                    <div class="user-box">
                        <h3>{"User 1"}</h3>
                        <p>{format!("Name: {}", *usr_name_1)}</p>
                    </div>
                }
            </form>
            <form class="row" onsubmit={usr_2}>
                <input id="usr-2-input" ref={usr_2_mmr_ref} />
                <button type="submit">{"User 2"}</button>
                if *show_box_2 {
                    <div class="user-box">
                        <h3>{"User 2"}</h3>
                        <p>{format!("Name: {}", *usr_name_2)}</p>
                    </div>
                }
            </form>
        </main>
    }
}