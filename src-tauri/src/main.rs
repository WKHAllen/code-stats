#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod code_stats;
mod dir_info;
mod types;

use code_stats::get_code_stats;
use dir_info::get_directory_info;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_code_stats, get_directory_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
