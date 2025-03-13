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
        self.daily_schedule(llama, date).await;
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
