// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod reg;





fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![reg::get_usb_keys, reg::delete_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
