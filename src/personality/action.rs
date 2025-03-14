use std::fmt::Display;

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
    location: (Region, Room),
    start_time: Time,
    intended_duration: i64,
    description: String,
    description_emoji: String,
    object: Option<MapObject>,
    //Chat
    chat: Option<Chat>,
    chat_target_buffer: Vec<String>,
}
impl Action {
    pub fn completed(&self, time: Time) -> bool {
        let (end_time, day) = self.start_time + Time::from_hms((0, 0, self.intended_duration));
        day > 0 || time >= end_time
    }
    pub fn to_string(&self) -> String {
        format!(
            "Currently {} at {}, {}.{}",
            self.description,
            self.location.0,
            self.location.1,
            match &self.object {
                Some(o) => format!(" {} {}.", o.action().clone().unwrap(), o.name()),
                None => String::new(),
            }
        )
    }
}
impl Default for Action {
    fn default() -> Self {
        Action {
            location: (Region::default(), Room::default()),
            start_time: Time::from_hms((0, 0, 0)),
            intended_duration: -1,
            description: String::new(),
            description_emoji: String::new(),
            object: None,
            chat: None,
            chat_target_buffer: Vec::new(),
        }
    }
}
pub enum ProperAction {
    SLEEP,
    WAKE,
}
impl Display for ProperAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SLEEP => "SLEEP",
                Self::WAKE => "WAKE",
                _ => "",
            }
        )
    }
}
#[derive(Clone)]
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
impl Display for ActionBare{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, From: {} To: {}", self.description, self.start, self.end)
    }
}
pub struct ActionBareVec(pub Vec<ActionBare>);

impl Display for ActionBareVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(", "))
    }
}
