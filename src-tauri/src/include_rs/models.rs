use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
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
