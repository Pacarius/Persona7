use std::fmt::Display;

use crate::{
    misc::time::{weekday, Date, DateTime, Time},
    personality::action::fmt_abv,
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
            name = self.name(),
            daily = self.short_term_mem().goals // daily = "".to_string()
        );
        source
    }
    pub fn vague(&self, date: &Date) -> String {
        let plan_vague = &self.short_term_mem().plan_vague;
        let source = format!(
            "{common}
            Today is {curr_date}.
            {name}starts his day at {wake} and ends it at {sleep}
            Here is a list of his plans today in broad strokes in the format of a json list of (Action_Description, Start_Time(HH:MM:SS form), End_Time(HH:MM:SS form))
            All actions are formatted as ongoing actions. Examples:[Working on personal project. Going for a run. Running around screaming like a schizophrenic racist.]
            ",
            common = self,
            name = self.name(),
            curr_date = date,
            wake = plan_vague.get(0).unwrap().end,
            sleep = plan_vague.get(1).unwrap().start
        );
        source
    }
    pub fn decompose(&self, datetime: &DateTime) -> String {
        let surrounding = self.short_term_mem().surrounding_tasks(datetime.1);
        // println!("{:?}", surrounding);
        let curr_acction = surrounding.get(1).unwrap();
        let source = format!(
            "{common}
            Today is {weekday} {date}.
            {name} is planning on {surrounding}
            In 900-second increments, list the subtasks that {name} does when {name} is {action} starting at {start_time}. (Total duration in seconds: {duration}).",
            common = self,
            name = self.name(),
            weekday = weekday(&datetime.0),
            date = datetime.0,
            surrounding = fmt_abv(surrounding),
            action = curr_acction.description,
            start_time = curr_acction.start,
            duration = (curr_acction.end - curr_acction.start).in_seconds()
        );
        source
    }
}
