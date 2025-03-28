use std::{error::Error, fmt::Display};

use futures::future::LocalBoxFuture;

use crate::{
    misc::time::{weekday, Date, DateTime, Time},
    personality::action::fmt_abv,
    world::{
        character::Character,
        world_map::{MapObject, WorldMap},
    },
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
    pub fn decompose(&self, datetime: &DateTime) -> Result<(String, i64, Time), Box<dyn Error>> {
        let surrounding = self.short_term_mem().surrounding_tasks(datetime.1);
        // println!("{:?}", surrounding);
        if let Some(curr_acction) = surrounding.get(1) {
            let duration = (curr_acction.end - curr_acction.start).in_seconds() / 60;
            let source = format!(
                "{common}
                Today is {weekday} {date}.
                {name} is planning on {surrounding}
                With durations in increments of 5 minutes, list the subtasks that {name} does when {name} is {action} starting at {start_time}. (Total duration in minutes: {duration}).
                Here is a sample: {sample}",
                common = self,
                name = self.name(),
                weekday = weekday(&datetime.0),
                date = datetime.0,
                surrounding = fmt_abv(surrounding),
                action = curr_acction.description,
                start_time = curr_acction.start,
                duration = duration,
                sample = "
                {
        \"Detailed_Tasks\": [
            {
                \"subtask_duration\": 15,
                \"remaining_duration\": 165,
                \"subtask_details\": \"Review kindergarten curriculum standards\"
            },
            {
                \"subtask_duration\": 30,
                \"remaining_duration\": 135,
                \"subtask_details\": \"Brainstorm ideas for the lesson\"
            },
            {
                \"subtask_duration\": 30,
                \"remaining_duration\": 105,
                \"subtask_details\": \"Create the lesson plan\"
            },
            {
                \"subtask_duration\": 30,
                \"remaining_duration\": 75,
                \"subtask_details\": \"Create materials for the lesson\"
            },
            {
                \"subtask_duration\": 15,
                \"remaining_duration\": 60,
                \"subtask_details\": \"Take a break\"
            },
            {
                \"subtask_duration\": 30,
                \"remaining_duration\": 30,
                \"subtask_details\": \"Review the lesson plan\"
            },
            {
                \"subtask_duration\": 15,
                \"remaining_duration\": 15,
                \"subtask_details\": \"Make final changes to the lesson plan\"
            }
        ]
    }
                "
            );
            // println!("{}", source);
            Ok((source, duration, curr_acction.start))
        } else {
            Err("Helper Error.".into())
        }
    }
    pub fn pick_object(
        &self,
        datetime: &DateTime,
        objects: &Vec<&String>,
    ) -> Result<String, Box<dyn Error>> {
        if let Some(current) = self.short_term_mem().surrounding_tasks(datetime.1).get(1) {
            let source = format!(
                    "{common}
                    {curr_object}
                    You are planning on {curr_action}
                    Here is a list of objects that you can see {object_list:?}.
                    If any objects are containers, you are suggested to automatically generate their contents.
                    Return a singular appropriate object for the event, or if no valid objects are present, return 'NONE'.",
                    common = self,
                    curr_object = match &self.short_term_mem().curr_object{
                        Some(o) => {format!(
                            "You are currently using {},",
                            o)},
                        None => {format!("")}
                    },
                    curr_action = current,
                    object_list = objects
                );
            return Ok(source);
        }
        Err("Helper Error".into())
    }
    //BASED ON VAGUE SCHEDULE
    pub fn ro(
        &self,
        datetime: &DateTime,
        // map: &WorldMap, // , map: &WorldMap
        location: (String, String),
    ) -> Result<String, Box<dyn Error>> {
        let surrounding = self.short_term_mem().surrounding_tasks(datetime.1);
        // println!("{:?}", surrounding);
        // println!("{:?}", self.spatial_mem());
        if let Some(curr_acction) = surrounding.get(1) {
            // let location = self.get_location(map);
            let source = format!(
                "{common}
                Today is {weekday} {date}.
                {name} is planning on {curr_action}
                Here are a list of known regions, rooms, and objects that are present on the map: {spatial} in the format of {{Regions: {{Rooms: {{Objects }}}}}}. You are currently in ({region}, {room}).
                Please decide on the most fitting location in the format of {{'region': {{}}, 'room': {{}}}} you want to execute the given action in.",
                common = self,
                name = self.name(),
                weekday = weekday(&datetime.0),
                date = datetime.0,
                curr_action = curr_acction.description,
                region = location.0,
                room = location.1,
                // action = curr_acction.description,
                spatial = self.spatial_mem(),
            );
            // println!("{}", source);
            Ok(source)
        } else {
            Err("Helper Error".into())
        }
    }
}
