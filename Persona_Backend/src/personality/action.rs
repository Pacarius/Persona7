use std::fmt::Display;

use futures::future::Map;
use serde::Serialize;

use crate::{
    misc::time::{Date, DateTime, Time},
    world::{
        character,
        utils::{MapObject, Region, Room},
        world_map::Coordinates,
    },
    TIME_STEP,
};
#[derive(Debug, Clone, Serialize)]
pub struct Chat {
    target: String,
    log: Vec<String>,
    end_time: i64,
}
#[derive(Debug, Clone, Serialize)]
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
        // println!("{} is past {}: {}", time, (self.start_time + Time::from_seconds(self.intended_duration)).0, )
        let (end_time, day) = self.start_time + Time::from_seconds(self.intended_duration);
        let output = day > 0 || time >= &end_time;
        // println!("{} is past {}: {}", time, end_time, output);
        output
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
#[derive(Clone, Debug, Serialize)]
// pub struct ActionEntry(pub String, pub Action, pub String);
pub struct ActionEntry {
    character_name: String,
    action: Action,
    object: Option<String>,
}
impl ActionEntry {
    pub fn new(character_name: String, action: Action, object: Option<String>) -> Self {
        Self {
            character_name,
            action,
            object,
        }
    }

    pub fn character_name(&self) -> String {
        self.character_name.clone()
    }

    pub fn action(&self) -> Action {
        self.action.clone()
    }

    pub fn object(&self) -> Option<String> {
        self.object.clone()
    }
}
// pub struct ActionEntry {
//     action_type: String,
//     character: String,
//     start_time: DateTime,
//     // In seconds
//     intended_duration: i64,
//     description: String,
//     object: Option<String>,
//     location: Coordinates
// }
// impl ActionEntry {
//     pub fn new(
//         action_type: String,
//         character: String,
//         start_time: DateTime,
//         intended_duration: i64,
//         description: String,
//         object: Option<String>,
//         location: Coordinates
//     ) -> Self {
//         Self {
//             action_type,
//             character,
//             start_time,
//             intended_duration,
//             description,
//             object,
//             location
//         }
//     }
//     pub fn movement(
//         character: String,
//         from: Coordinates,
//         to: Coordinates,
//         curr_time: DateTime,
//     ) -> Self {
//         ActionEntry::new(
//             ProperAction::MOVE.to_string(),
//             character,
//             curr_time,
//             TIME_STEP,
//             format!("MOVE|{}|{}", from, to),
//             None,
//             from
//         )
//     }
//     pub fn character(&self) -> String {
//         self.character.clone()
//     }
// }
// impl Display for ActionEntry {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}|{}|{}|{}|{}",
//             self.action_type,
//             self.character,
//             self.start_time,
//             self.intended_duration,
//             self.description
//         )
//     }
// }
// fn to_entry(action: &Action, character: &String, )
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
