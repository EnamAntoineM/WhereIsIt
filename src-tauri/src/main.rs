// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod wrappers;
mod include_rs;
use crate::wrappers::*;

fn main() {
    // Step 1: Call init_db() function to make sure the DB file/folder exists
    let _ = include_rs::db::init_db();

    // Step 2: Start the Tauri app
    tauri::Builder::default()
        // Step 3: Register the backend commands exposed to the frontend
        .invoke_handler(tauri::generate_handler![
            get_all_items_cmd,
            add_item_cmd,
            search_element_cmd,
            create_table_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

