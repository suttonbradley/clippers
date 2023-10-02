// TODO: delete? Would need this Rust code:
// #[wasm_bindgen(module = "/public/glue.js")]
// extern "C" {
//     #[wasm_bindgen(js_name = executeQuery, catch)]
//     pub fn execute_query(name: String) -> Result<JsValue, JsValue>;
// }

const invoke = window.__TAURI_INVOKE__

export function executeQuery(query) {
    return invoke("execute_query", { query: query });
}
