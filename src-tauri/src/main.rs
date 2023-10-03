// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::Manager;
use log::info;
use {env_logger, libclippers};

#[tauri::command]
// TODO: make async as generated?
fn execute_query(query: &str) -> String {
    info!("Querying: \"{query}\"");
    libclippers::get_matches(query);
    format!("YOU QUERIED {query}")
}

// #[derive(Default)]
// struct MyState {
//   s: std::sync::Mutex<String>,
//   t: std::sync::Mutex<std::collections::HashMap<String, String>>,
// }
// // remember to call `.manage(MyState::default())`
// #[tauri::command]
// async fn command_name(state: tauri::State<'_, MyState>) -> Result<(), String> {
//   *state.s.lock().unwrap() = "new string".into();
//   state.t.lock().unwrap().insert("key".into(), "value".into());
//   Ok(())
// }

fn main() {
    // TODO: turn off for release builds?
    env_logger::init();
    libclippers::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
