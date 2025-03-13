use std::fmt::Display;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    misc::{
        ollama::{
            ollama::Ollama,
            options::{FormatPair, FormatTriple, GenerateOptions},
        },
        time::{Date, Time, DAY_LENGTH},
    },
    personality::{
        action::{ActionBare, ProperAction},
        memory::short_term::ShortTerm,
    },
    TEXT_MODEL,
};

use super::world_map::Coordinates;

// use super::world::WorldListener;

#[derive(Debug, Deserialize)]
pub enum Placeholder {
    MALE,
    FEMALE,
}

#[derive(Debug, Deserialize)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

// #[derive(Debug)]
pub struct Character {
    // first_name: String,
    // last_name: String,
    age: i64,
    core_traits: Vec<String>,
    stable_traits: Vec<String>,
    // current_action: String,
    lifestyle: String,
    living_area: (String, String),
    short_term_mem: ShortTerm,
    sprite: Placeholder,
    pub name: String,
    pub location: Coordinates,
    // path: Option<Vec<Coordinates>>,
    direction: Direction,
    // pub daily_tasks: Vec<String>,
}

impl Character {
    pub fn new(
        age: i64,
        core_traits: Vec<String>,
        stable_traits: Vec<String>,
        lifestyle: String,
        living_area: (String, String),
        short_term_mem: ShortTerm,
        sprite: Placeholder,
        name: String,
        location: Coordinates,
        path: Option<Vec<Coordinates>>,
        direction: Direction,
        daily_tasks: Vec<String>,
    ) -> Self {
        Character {
            name,
            sprite,
            location,
            // path: None,
            direction,
            age,
            core_traits,
            stable_traits,
            lifestyle,
            living_area,
            short_term_mem,
            // daily_tasks,
        }
    }
    pub fn short_term_mem(&self) -> &ShortTerm {
        &self.short_term_mem
    }
    pub fn short_term_mem_mut(&mut self) -> &mut ShortTerm {
        &mut self.short_term_mem
    }
    pub fn tick(&self, time: &crate::misc::time::Time) {}
    //Characters ticking : New Day; Activity Ended; if Activity is moving from point A to point B.
    pub async fn day_start(&mut self, llama: &Ollama, date: &Date) {
        self.wake_time(llama).await;
        // self.daily_schedule(llama, date).await;
    }
    async fn wake_time(&mut self, llama: &Ollama) {
        let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), self.rest_wake());
        options.format_pair(vec![
            FormatPair("wake_time".to_string(), &json!("string")),
            FormatPair("sleep_time".to_string(), &json!("string")),
        ]);

        if let Some(response_str) = llama.generate(options).await["response"].as_str() {
            if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                if let Some(wake_time_str) = response_json["wake_time"].as_str() {
                    if let Some(wake_time) = Time::parse_time_pair(wake_time_str) {
                        let wake_action = ActionBare::new(ProperAction::WAKE.to_string(), Time::from_seconds(0), wake_time.0);
                        self.short_term_mem.plan_vague.clear();
                        self.short_term_mem.plan_vague.push(wake_action);
                    }
                }
                if let Some(sleep_time_str) = response_json["sleep_time"].as_str() {
                    if let Some(sleep_time) = Time::parse_time_pair(sleep_time_str) {
                        let sleep_action = ActionBare::new(ProperAction::SLEEP.to_string(), sleep_time.0, Time::from_seconds(DAY_LENGTH - 1));
                        self.short_term_mem.plan_vague.push(sleep_action);
                    }
                }
            }
        }
    }

    async fn daily_schedule(&mut self, llama: &Ollama, date: &Date) {
        let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), self.vague(date));
        options.format_triple(FormatTriple("actions".to_string(), vec![
            FormatPair("action_description".to_string(), &json!("string")),
            FormatPair("start_time".to_string(), &json!("string")),
            FormatPair("end_time".to_string(), &json!("string")),
        ]));

        if let Some(response_str) = llama.generate(options).await["response"].as_str() {
            if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                let mut plans = vec![];

                for plan in response_json.as_array().unwrap_or(&vec![]) {
                    if let (Some(action_desc), Some(start_time), Some(end_time)) = (
                        plan["action_description"].as_str(),
                        plan["start_time"].as_str(),
                        plan["end_time"].as_str(),
                    ) {
                        if let (Some(start), Some(end)) = (
                            Time::parse_time_pair(start_time),
                            Time::parse_time_pair(end_time),
                        ) {
                            let action_bare = ActionBare::new(action_desc.to_string(), start.0, end.0);
                            plans.push(action_bare);
                        }
                    }
                }

                // Assuming the first and last entries in plan_vague are wake and sleep actions
                if let Some(wake_action) = self.short_term_mem.plan_vague.first().cloned() {
                    if let Some(sleep_action) = self.short_term_mem.plan_vague.last().cloned() {
                        self.short_term_mem.plan_vague.clear();
                        self.short_term_mem.plan_vague.push(wake_action);
                        self.short_term_mem.plan_vague.extend(plans);
                        self.short_term_mem.plan_vague.push(sleep_action);
                    }
                }
            }
        }
    }
}
impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} \n",
            format!(
                "{name} is {age}. {name} is {core}, {stable}. {lifestyle}",
                name = self.name,
                age = self.age,
                core = self.core_traits.join(","),
                stable = self.stable_traits.join(","),
                lifestyle = self.lifestyle
            )
        )
    }
}
