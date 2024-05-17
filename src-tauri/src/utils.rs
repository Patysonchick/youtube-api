use std::fs;
use std::env;

const TOKEN_PATH: &str = ".token";

#[tauri::command]
pub fn get_token() -> Result<String, ()> {
    let file = fs::read_to_string(TOKEN_PATH); // .expect("can`t read file")
    match file {
        Ok(string) => {
            // println!("Readed token: {}", string);
            Ok(string)
        },
        Err(_) => {
            println!("No token");
            Err(())
        }
    }
}

#[tauri::command]
pub fn get_pkg_version() -> String {
    let version = env!("CARGO_PKG_VERSION").to_string();
    println!("Pkg version: {}", version);

    version
}
