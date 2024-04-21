# youtube-api
Rust + Tauri application for testing YouTube API

# Functions
1. Getting views
2. Coming soon...

# Installation
## Rust:
* If you are on Windows - https://www.rust-lang.org/tools/install
* Either Linux or Unix-like + macOS - run in terminal/console `curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh`

Check installation by running in terminal/console `cargo --version`.
Example normal output `cargo 1.75.0 (1d8b05cdd 2023-11-20)`

## After Rust successful installed
Run in terminal/console
1. Install Prerequisites from https://tauri.app/v1/guides/getting-started/prerequisites
2. `cargo install create-tauri-app --locked`
3. `cargo install tauri-cli`

# Testing
1. `cd youtube-api`
2. `cargo tauri dev`.
If you want to make installers or optimized portable binary run `cargo tauri build` 
3. enjoy :)
