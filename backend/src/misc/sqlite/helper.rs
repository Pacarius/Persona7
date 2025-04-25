use std::fs;
use rusqlite::{Connection, Result, params};
use serde_json::Value;
use crate::CONFIG;

pub struct Helper {
    path: String,
    conn: Connection,
}

impl Helper {
    pub fn new() -> Result<Self> {
        // Get base path from config
        let base_path = &CONFIG.sqlite_path;
        
        // Ensure directory exists
        if let Err(e) = fs::create_dir_all(base_path) {
            eprintln!("Failed to create directory {}: {}", base_path, e);
        }
        
        // Count existing .db files
        let count = match fs::read_dir(base_path) {
            Ok(entries) => entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.path().extension()
                        .map_or(false, |ext| ext == "db")
                })
                .count(),
            Err(e) => {
                eprintln!("Failed to read directory {}: {}", base_path, e);
                0
            }
        };
        
        // Generate new path: base_path + next available number + .db
        let path = format!("{}/{}.db", base_path, count + 1);
        
        // Create connection
        let conn = Connection::open(&path)?;
        
        let helper = Self { path, conn };
        
        // Initialize database tables
        helper.init_db()?;
        
        Ok(helper)
    }
    
    // Initialize the database by creating the required table
    fn init_db(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                type TEXT NOT NULL,
                timestamp TEXT,
                content BLOB NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }
    
    // Add an entry to the database from JSON string
    pub fn add_entry(&self, json_str: &str) -> Result<()> {
        // Parse the JSON string
        let json_value: Value = serde_json::from_str(json_str)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        
        // Extract values
        let entry_type = json_value["type"].as_str()
            .ok_or_else(|| rusqlite::Error::InvalidParameterName("Missing 'type' field".into()))?;
        
        let content = json_value["content"].to_string();
        
        // Timestamp might be None for "init" entries
        let timestamp = if json_value.get("timestamp").is_some() && !json_value["timestamp"].is_null() {
            Some(json_value["timestamp"].as_str()
                .ok_or_else(|| rusqlite::Error::InvalidParameterName("Invalid 'timestamp' field".into()))?)
        } else {
            None
        };
        
        // Insert into database
        self.conn.execute(
            "INSERT INTO events (type, timestamp, content) VALUES (?1, ?2, ?3)",
            params![entry_type, timestamp, content],
        )?;
        
        Ok(())
    }
    
    // Get the path of the database file
    pub fn get_path(&self) -> &str {
        &self.path
    }
}