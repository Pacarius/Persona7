// use std::{collections::HashMap, fmt::Display};

// pub enum DBDataType {
//     NULL,
//     INTEGER,
//     REAL,
//     TEXT,
//     BLOB,
// }
// impl Display for DBDataType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 DBDataType::NULL => "NULL",
//                 DBDataType::BLOB => "BLOB",
//                 DBDataType::INTEGER => "INTEGER",
//                 DBDataType::REAL => "REAL",
//                 DBDataType::TEXT => "TEXT"
//             }
//         )
//     }
// }
// pub trait DBData {
//     fn table(&self, data: DBDataMap) -> String;
// }
// pub struct DBDataMap(pub HashMap<String, DBDataType>);

use crate::{
    misc::time::{DateTime, Time},
    world::world_map::Coordinates,
};

// impl Display for DBDataMap {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let formatted: Vec<String> = self.0.iter()
//             .map(|(k, v)| format!("{} {}", k, v))
//             .collect();
//         write!(f, "({})", formatted.join(", "))
//     }
// }
pub struct Movement {
    character: String,
    from: Coordinates,
    to: Coordinates,
    timestamp: DateTime,
}
impl Movement {
    pub fn new(character: String, from: Coordinates, to: Coordinates, timestamp: DateTime) -> Self {
        Self {
            character,
            from,
            to,
            timestamp,
        }
    }
    pub fn to_insert(&self) -> String {
        format!(
            "INSERT INTO movement (character, from, to, timestamp) values ({}, {}, {}, {})",
            self.character, self.from, self.to, self.timestamp
        )
    }
}
pub struct Action {
    character: String,
    description: String,
    object: Option<String>,
    timestamp: DateTime,
    duration: Time,
}
