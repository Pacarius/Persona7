// use crate::sqlite::data::DBData;
use std::{fmt::Debug, fs};

// use reqwest::redirect::Action;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

use crate::world::world_map::WorldMap;

use super::data::{Action, Movement};

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
    fn load(&mut self) -> (WorldMap, Vec<Action>, Vec<Movement>) {
        todo!()
    }
    // pub fn create(&self, data: &impl DBData) {}
    // pub fn write(&self, data: &(impl Serialize + Debug)) {
    //     // self.conn.execute()
    // }
}
pub struct DBController {}
impl DBController {
    fn character_locations() {}
}
