use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MajItem {
    pub id: i32,
    pub name: String,
    pub units: i32,
    pub registered_at: String,
    pub last_modified: Option<String>,
    pub expiry_date: Option<String>,
    pub location: String,
    pub note: Option<String>,
    pub visual: Option<String>,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub units: i32,
    pub registered_at: String,
    pub last_modified: Option<String>,
    pub expiry_date: Option<String>,
    pub location: Option<String>,
    pub note: Option<String>,
    pub visual: Option<String>,
    pub owner: String,
}
