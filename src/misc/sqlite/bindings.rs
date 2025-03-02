// use crate::sqlite::data::DBData;
use std::{fmt::Debug, fs};

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

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
    pub fn create(&self, data: &impl DBData) {}
    pub fn write(&self, data: &(impl Serialize + Debug)) {
        // self.conn.execute()
    }
}
pub struct DBController {}
impl DBController {
    fn character_locations() {}
}
