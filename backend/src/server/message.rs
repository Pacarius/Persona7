// use std::fmt::{self, Display};

// use serde_json::Value;

// use crate::misc::time::DateTime;

// pub struct Message {
//     message_type: MessageType,
//     content: String,
//     timestamp: Option<DateTime>,
// }
// impl Message {
//     pub fn new(message_type: MessageType, content: String, timestamp: Option<DateTime>) -> Self {
//         Self {
//             message_type,
//             content,
//             timestamp,
//         }
//     }
//     pub fn to_json(&self) -> Value {
//         serde_json::json!({
//             "type": self.message_type.to_string(),
//             "content": self.content,
//             "timestamp": match &self.timestamp{
//                 Some(ts) => ts.to_string(),
//                 None => "NONE".into()
//             },
//         })
//         // .to_string()
//     }
// }
// impl Display for Message {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_json().to_string())
//     }
// }
// #[derive(Debug)]
// pub enum MessageType {
//     WEB,
//     PY,
// }
// impl fmt::Display for MessageType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }
