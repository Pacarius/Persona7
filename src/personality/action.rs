use std::fmt::Display;

use futures::future::Map;

use crate::{
    misc::time::Time,
    world::world_map::{MapObject, Region, Room},
};
#[derive(Debug)]
pub struct Chat {
    target: String,
    log: Vec<String>,
    end_time: i64,
}
#[derive(Debug)]
pub struct Action {
    location: (String, String),
    start_time: Time,
    intended_duration: i64,
    description: String,
    // description_emoji: String,
    object: Option<String>,
    //Chat
    chat: Option<Chat>,
    // cleanup: Option<Box<dyn Fn() -> ()>>, // chat_target_buffer: Vec<String>,
}
impl Action {
    pub fn completed(&self, time: &Time) -> bool {
        // if self.intended_duration < 0 {return true}
        let (end_time, day) = self.start_time + Time::from_seconds(self.intended_duration);
        day > 0 || time >= &end_time
    }
    fn d(&self) -> String {
        format!(
            "Start Time: {}.\n Intended Duration: {}.\n Currently {} at {}, {}.{}",
            self.start_time,
            self.intended_duration,
            self.description,
            self.location.0,
            self.location.1,
            match &self.object {
                Some(o) => o.clone(),
                None => String::new(),
            }
        )
    }
    // pub fn cleanup(&mut self){
    //     self.cleanup
    // }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn new(
        location: (String, String),
        start_time: Time,
        intended_duration: i64,
        description: String,
        // description_emoji: String,
        object: Option<String>,
        chat: Option<Chat>,
        // cleanup: Option<Box<dyn Fn() -> ()>>, // chat_target_buffer: Vec<String>,
    ) -> Self {
        Self {
            location,
            start_time,
            intended_duration,
            description,
            // description_emoji,
            object,
            chat,
            // cleanup, // chat_target_buffer,
        }
    }
    pub fn object(&self) -> Option<String> {
        self.object.clone()
    }
}
// impl Default for Action {
//     fn default() -> Self {
//         Action {
//             location: (Region::default(), Room::default()),
//             start_time: Time::from_hms((0, 0, 0)),
//             intended_duration: -1,
//             description: String::new(),
//             description_emoji: String::new(),
//             object: None,
//             chat: None,
//             chat_target_buffer: Vec::new(),
//         }
//     }
// }
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.d())
    }
}
pub enum ProperAction {
    SLEEP,
    WAKE,
    MOVE,
    TALK,
}
impl Display for ProperAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SLEEP => "SLEEP",
                Self::WAKE => "WAKE",
                Self::MOVE => "MOVE",
                Self::TALK => "TALK",
                _ => "",
            }
        )
    }
}
#[derive(Clone, Debug)]
pub struct ActionBare {
    pub description: String,
    pub start: Time,
    pub end: Time,
}
impl ActionBare {
    pub fn new(description: String, start: Time, end: Time) -> Self {
        ActionBare {
            description,
            start,
            end,
        }
    }
}
impl Display for ActionBare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "From {} To {}: {}",
            self.start, self.end, self.description
        )
    }
}
pub fn fmt_abv(abv: &[ActionBare]) -> String {
    format!(
        "[{}]",
        abv.iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}
