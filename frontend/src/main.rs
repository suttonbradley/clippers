use serde::Serialize;
use serde_wasm_bindgen::to_value as to_js_value;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use log::debug;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Serialize)]
struct Query<'a> {
    query: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    // Init logger
    // TODO: turn off for release builds?
    wasm_logger::init(wasm_logger::Config::default());
    debug!("Logger initialized");

    let query_results = use_state_eq(String::default);

    // Callback that executes query on every change to input
    // TODO: make async?
    let on_change_cb = {
        let query_results = query_results.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                query_results.set(
                    invoke(
                        "execute_query",
                        to_js_value(&Query {
                            query: input.value().as_str(),
                        })
                        .unwrap(),
                    )
                    .as_string()
                    .unwrap(),
                )
            }
        })
    };

    html! {
        <div>
            <input oninput={on_change_cb}/>
            <p><b>{ &*query_results }</b></p>
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = __TAURI_INVOKE__)]
    fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
