use std::{collections::VecDeque, fmt::Display};

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
        memory::{short_term::ShortTerm, spatial::SpatialMemory},
    },
    TEXT_MODEL,
};

use super::world_map::{Coordinates, Room, WorldMap};

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
    spatial: SpatialMemory,
    sprite: Placeholder,
    name: String,
    position: Coordinates,
    location: Room,
    path: Option<VecDeque<Coordinates>>,
    view_range: i64, // direction: Direction,
                     // pub daily_tasks: Vec<String>,
}

impl Character {
    pub fn new(
        age: i64,
        core_traits: Vec<String>,
        stable_traits: Vec<String>,
        lifestyle: String,
        living_area: String,
        // short_term_mem: ShortTerm,
        sprite: Placeholder,
        name: String,
        position: Coordinates,
        // path: Option<Vec<Coordinates>>,
        // direction: Direction,
        // daily_tasks: Vec<String>,
        view_range: i64,
    ) -> Self {
        Character {
            name,
            sprite,
            position,
            location: Room::default(),
            path: None,
            // direction,
            age,
            core_traits,
            stable_traits,
            lifestyle,
            living_area,
            short_term_mem: ShortTerm::default(),
            spatial: SpatialMemory::default(),
            view_range, // daily_tasks,h
        }
    }
    pub fn ascend(&mut self, map: &WorldMap){
        self.spatial = SpatialMemory::god(map);
    }
    pub fn short_term_mem(&self) -> &ShortTerm {
        &self.short_term_mem
    }
    pub fn short_term_mem_mut(&mut self) -> &mut ShortTerm {
        &mut self.short_term_mem
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn position(&self) -> &Coordinates {
        &self.position
    }
    pub fn v_range(&self) -> &i64 {
        &self.view_range
    }
    pub fn path(&self) -> &Option<VecDeque<Coordinates>> {
        &self.path
    }
    pub fn _move(&mut self) -> Option<(Coordinates, Coordinates)> {
        if let Some(path) = &mut self.path {
            if path.len() >= 2 {
                let (from, to) = (path[0].clone(), path[1].clone());
                path.pop_front();
                //Add to database
                // todo!();
                self.position = to.clone();
                Some((from, to))
            } else {
                path.clear();
                None
            }
        } else {
            None
        }
    }
    pub fn set_path(&mut self, path: VecDeque<Coordinates>) {
        self.path = Some(path)
    }
    //Characters ticking : New Day; Activity Ended; if Activity is moving from point A to point B.
    pub async fn day_start(&mut self, llama: &Ollama, date: &Date) {
        self.wake_time(llama).await;
        self.daily_schedule(llama, date).await;
    }
    pub async fn tick(&mut self, time: &crate::misc::time::Time) {}
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
