use std::fmt::Display;

use crate::{
    misc::time::{Date, DateTime, Time},
    world::character::Character,
};
impl Character {
    pub fn rest_wake(&self) -> String {
        let source = format!(
            "{common}
    Daily Plan: Today, {name} is planning to do the following five things: {daily:?}
    Return {name}'s waking and sleeping time, both in HH:MM:SS form.",
            common = self,
            // common = "".to_string(),
            name = self.name,
            daily = self.short_term_mem().goals // daily = "".to_string()
        );
        source
    }
    pub fn vague(&self, date: &Date) -> String {
        let plan_vague = &self.short_term_mem().plan_vague.0;
        let source = format!(
            "{common}
            Today is {curr_date}.
            {name}starts his day at {wake} and ends it at {sleep}
            Here is a list of his plans today in broad strokes in the format of a json list of (Action_Description, Start_Time(HH:MM:SS form), End_Time(HH:MM:SS form))
            ",
            common = self,
            name = self.name,
            curr_date = date,
            wake = plan_vague.get(0).unwrap().end,
            sleep = plan_vague.get(1).unwrap().start
        );
        source
    }
}
