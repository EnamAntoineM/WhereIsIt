use crate::include_rs::db::*;
use crate::include_rs::models::*;
use rusqlite::Result;
use tauri::command;

#[command]
pub fn get_all_items_cmd(table: String) -> Result<Vec<MajItem>, String> {
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
pub fn search_element_cmd(element: String, table_name: String) -> Result<Vec<MajItem>, String> {
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

#[command]
pub fn update_item_cmd(
    item: MajItem,
    item_id: i32,
    current_table: String,
) -> Result<String, String> {
    match update_item(&item, &item_id, &current_table) {
        Ok(_) => Ok("Item(s) updated successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn delete_item_cmd(item_id: i32, current_table: String) -> Result<String, String> {
    match delete_item(&item_id, &current_table) {
        Ok(_) => Ok("Item(s) deleted successfully".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
