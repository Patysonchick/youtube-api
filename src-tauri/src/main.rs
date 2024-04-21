#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get;

use get::get_views;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_views])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
