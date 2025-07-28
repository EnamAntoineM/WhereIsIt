use crate::include_rs::models::*;
use rusqlite::{params, Connection, Result};

pub fn init_db() -> Result<Connection> {
    let mut path = dirs::data_dir().unwrap_or_else(|| std::env::temp_dir());
    path.push("whereisit");
    std::fs::create_dir_all(&path)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    path.push("database.sqlite");
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn create_table(table_name: &str) -> Result<()> {
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let conn = init_db()?;
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        units INTEGER NOT NULL,
        registered_at TEXT NOT NULL,
        last_modified TEXT,
        expiry_date TEXT,
        location TEXT,
        note TEXT,
        visual TEXT,
        owner TEXT
    );",
        table_name
    );
    conn.execute(&sql, [])?;
    Ok(())
}

pub fn search_element(element: &str, table_name: &str) -> Result<Vec<MajItem>> {
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(rusqlite::Error::InvalidQuery);
    }
    if !element
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == ' ')
    {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let sql = format!(
        "SELECT * FROM \"{}\" WHERE name LIKE ?1 || '%';",
        table_name
    );
    let conn = init_db()?;
    let mut stmt = conn.prepare(&sql)?;
    let query_res = stmt.query_map([element], |row| {
        Ok(MajItem {
            id: row.get(0)?,
            name: row.get(1)?,
            units: row.get(2)?,
            registered_at: row.get(3)?,
            last_modified: row.get(4)?,
            expiry_date: row.get(5)?,
            location: row.get(6)?,
            note: row.get(7)?,
            visual: row.get(8)?,
            owner: row.get(9)?,
        })
    })?;
    let mut results = Vec::new();
    for item in query_res {
        results.push(item?);
    }

    Ok(results)
}

pub fn add_item(item: &Item, current_table: &str) -> Result<()> {
    if !current_table
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let conn = init_db()?;
    let sql = format!(
        "INSERT INTO \"{}\" (name, units, registered_at, last_modified, expiry_date, location, note, visual, owner)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", current_table
    );
    conn.execute(
        &sql,
        params![
            item.name,
            item.units,
            item.registered_at,
            item.last_modified,
            item.expiry_date,
            item.location,
            item.note,
            item.visual,
            item.owner,
        ],
    )?;
    Ok(())
}

pub fn update_item(item: &MajItem, item_id: &i32, current_table: &str) -> Result<()> {
    if !current_table
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let conn = init_db()?;
    let sql = format!(
        "UPDATE \"{}\" SET 
            name = ?1,
            units = ?2,
            registered_at = ?3,
            last_modified = ?4,
            expiry_date = ?5,
            location = ?6,
            note = ?7,
            visual = ?8,
            owner = ?9
         WHERE id = ?10",
        current_table
    );
    conn.execute(
        &sql,
        params![
            item.name,
            item.units,
            item.registered_at,
            item.last_modified,
            item.expiry_date,
            item.location,
            item.note,
            item.visual,
            item.owner,
            item_id,
        ],
    )?;
    Ok(())
}

pub fn delete_item(item_id: &i32, current_table: &str) -> Result<()> {
    if !current_table
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let conn = init_db()?;
    let sql = format!("DELETE FROM \"{}\" WHERE id = ?1", current_table);
    conn.execute(&sql, params![item_id])?;
    Ok(())
}

pub fn delete_table(table_name: &str) -> Result<()> {
    if !table_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let conn = init_db()?;
    let sql = format!("DROP TABLE \"{}\"", table_name);
    conn.execute(&sql, [])?;
    Ok(())
}

pub fn get_all_items(current_view: &str) -> Result<Vec<MajItem>> {
    let conn = init_db()?;
    if !current_view
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(rusqlite::Error::InvalidQuery);
    }
    let sql = format!("SELECT * FROM \"{}\"", current_view);
    let mut stmt = conn.prepare(&sql)?;

    let item_iter = stmt.query_map([], |row| {
        Ok(MajItem {
            id: row.get(0)?,
            name: row.get(1)?,
            units: row.get(2)?,
            registered_at: row.get(3)?,
            last_modified: row.get(4)?,
            expiry_date: row.get(5)?,
            location: row.get(6)?,
            note: row.get(7)?,
            visual: row.get(8)?,
            owner: row.get(9)?,
        })
    })?;

    let mut items = Vec::new();
    for item in item_iter {
        items.push(item?);
    }
    Ok(items)
}
