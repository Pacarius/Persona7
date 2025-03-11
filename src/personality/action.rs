use crate::{misc::time::Time, world::world_map::{MapObject, Region, Room}};
#[derive(Debug)]
pub struct Chat{
    target: String,
    log: Vec<String>,
    end_time: i64
}

// #[derive(Debug)]
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
            chat_target_buffer: Vec::new()
        }
    }
}
