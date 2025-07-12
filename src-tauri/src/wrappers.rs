use rusqlite::Result;
use tauri::command;
use crate::include_rs::models::Item;
use crate::include_rs::db::*;

#[command]
pub fn get_all_items_cmd(table: String) -> Result<Vec<Item>, String> {
    match get_all_items(&table) {
        Ok(items) => Ok(items),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn add_item_cmd(item: Item, current_table: String) -> Result<String, String> {
    match add_item(&item, &current_table) {
        Ok(_) => Ok("Item added successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn search_element_cmd(element: String, table_name: String) -> Result<Vec<Item>, String> {
    match search_element(&element, &table_name) {
        Ok(items) => Ok(items),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn create_table_cmd(table_name: String) -> Result<String, String> {
    match create_table(&table_name) {
        Ok(_) => Ok("Table created successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}