// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ethereum;
use ethereum::address;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![address::public_key_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
