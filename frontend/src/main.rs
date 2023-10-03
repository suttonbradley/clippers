use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use log::debug;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    // Init logger
    // TODO: turn off for release builds?
    wasm_logger::init(wasm_logger::Config::default());
    debug!("Logger initialized");

    // TODO: use_state_eq?
    let query_results_handle = use_state(QueryResultListProps::default);

    // Callback that executes query on every change to input
    // TODO: make async?
    let on_change_cb = {
        Callback::from(move |e: InputEvent| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                // TODO: try not to copy here
                let input = input.value();
                query_results_handle.set(QueryResultListProps { results: vec![QueryResult { res: input }] });
            }
        })
    };

    html! {
        <div>
            <input oninput={on_change_cb}/>
            <QueryResultList results={ &*query_results_handle } />
        </div>
    }
}

#[function_component(QueryResultList)]
fn query_result(QueryResultListProps { results }: &QueryResultListProps) -> Html {
    html! {
        <ul>
            { results.iter().map(|x| x.to_html()).collect::<Vec<Html>>() }
        </ul>
    }
}

#[derive(Default, Properties, PartialEq)]
struct QueryResultListProps {
    results: Vec<QueryResult>,
}

#[derive(PartialEq)]
struct QueryResult {
    res: String,
}

impl ToHtml for QueryResult {
    fn to_html(&self) -> Html {
        html! { <li>{ self.res }</li> }
    }
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = window, js_name = __TAURI_INVOKE__)]
//     fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }
