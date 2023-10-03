// use libclippers;
use log::info;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn main() {
    // Init logger
    // TODO: turn off for release builds?
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logger initialized");

    // libclippers::init();
    // info!("Initialized libclippers"); // TODO: remove

    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    // TODO: use_state_eq?
    let query_results_handle = use_state(QueryResultListProps::default);

    // Callback that executes query on every change to input
    // TODO: make async?
    let on_change_cb = {
        let query_results_handle = query_results_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                // TODO: try not to copy here
                let input = input.value();
                query_results_handle.set(QueryResultListProps {
                    // TODO: inefficient -- make the types compatible (ideally shared memory too)
                    // results: libclippers::get_matches(&input).into_iter().map(QueryResult::from).collect(),
                    results: vec![QueryResult::from(input)]
                });
            }
        })
    };

    html! {
        <div>
            <input oninput={on_change_cb}/>
            <QueryResultList ..(*query_results_handle).clone() />
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

#[derive(Default, Properties, PartialEq, Clone)]
struct QueryResultListProps {
    results: Vec<QueryResult>,
}

#[derive(PartialEq, Clone)]
struct QueryResult {
    res: AttrValue,
}

impl QueryResult {
    fn from(s: String) -> Self {
        QueryResult { res: s.into() }
    }
}

impl ToHtml for QueryResult {
    fn to_html(&self) -> Html {
        html! { <li>{ self.res.clone() }</li> }
    }
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = window, js_name = __TAURI_INVOKE__)]
//     fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }
