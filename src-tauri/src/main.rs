#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get;
mod utils;

use get::get_views;
use utils::{get_pkg_version, get_token};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_views, get_pkg_version, get_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
