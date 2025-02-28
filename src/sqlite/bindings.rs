use std::fs;

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use serde_rusqlite::{from_rows, to_params_named};

pub struct DBConnector {
    conn: Connection,
    name: String,
}
pub trait DBData: Serialize + for<'a> Deserialize<'a> + std::fmt::Debug {}
impl DBConnector {
    pub fn new(name: String) -> Result<DBConnector> {
        let folder = "./simulations";
        if !fs::exists(folder).unwrap() {
            fs::create_dir(folder);
        }
        let location = format!("{}/{}.db", folder, name);
        let conn = match Connection::open(location) {
            Ok(conn) => conn,
            Err(e) => return Err(e),
        };
        Ok(DBConnector { name, conn })
    }
    pub fn write(data: &impl DBData) {}
}
