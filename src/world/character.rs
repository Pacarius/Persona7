use std::{collections::VecDeque, fmt::Display};

use rayon::option;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{misc::ollama::{ollama::Ollama, options::{FormatPair, GenerateOptions}}, personality::memory::short_term::{Path, ShortTerm}};

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
    living_area: String,
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
        living_area: String,
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
    pub fn short_term_mem(&self) -> &ShortTerm {&self.short_term_mem}
    pub fn short_term_mem_mut(&mut self) -> &mut ShortTerm {&mut self.short_term_mem}

    pub fn get_descriptor(&self) -> String {
        format!(
            "{name} is {age}. {name} is {core}, {stable}. {lifestyle}",
            name = self.name,
            age = self.age,
            core = self.core_traits.join(","),
            stable = self.stable_traits.join(","),
            lifestyle = self.lifestyle
        )
    }
    pub fn tick(&self, time: &crate::misc::time::Time) {
    }
    //Characters ticking : New Day; Activity Ended; if Activity is moving from point A to point B.
    pub fn day_start(&self) {
        
    }
    pub async fn day_start_str(&self, llama: &Ollama) -> String {
        let mut options = GenerateOptions::new("llama3.2".to_string(), self.rest());
        options.format(vec![FormatPair("time".to_string(), &json!("string"))]);
        let response = llama.generate(options).await;
        let response_str = response["response"].as_str().unwrap_or("");
        let response_json: Value = serde_json::from_str(response_str).unwrap_or(Value::Null);
        response_json["time"].as_str().unwrap_or("").to_string()
    }
}
impl Display for Character{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \n", self.get_descriptor())
    }
}
