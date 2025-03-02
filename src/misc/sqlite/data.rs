use std::{collections::HashMap, fmt::Display};

pub enum DBDataType {
    NULL,
    INTEGER,
    REAL,
    TEXT,
    BLOB,
}
impl Display for DBDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DBDataType::NULL => "NULL",
                DBDataType::BLOB => "BLOB",
                DBDataType::INTEGER => "INTEGER",
                DBDataType::REAL => "REAL",
                DBDataType::TEXT => "TEXT"
            }
        )
    }
}
pub trait DBData {
    fn table(&self, data: DBDataMap) -> String;
}
pub struct DBDataMap(pub HashMap<String, DBDataType>);

impl Display for DBDataMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted: Vec<String> = self.0.iter()
            .map(|(k, v)| format!("{} {}", k, v))
            .collect();
        write!(f, "({})", formatted.join(", "))
    }
}