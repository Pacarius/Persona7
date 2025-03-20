use std::{collections::VecDeque, fmt::Display};

use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    misc::{
        ollama::{
            ollama::Ollama,
            options::{FormatPair, FormatTriple, GenerateOptions},
        },
        time::{Date, DateTime, Time, DAY_LENGTH},
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
    // location: Option<Room>,
    path: Option<VecDeque<Coordinates>>,
    view_range: i64, 
    movement_cooldown_input: i64, //Extra number for inputting update to movement ratio.
    movement_cooldown: i64
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
        movement_cooldown_input: i64
    ) -> Self {
        Character {
            name,
            sprite,
            position,
            // location: None,
            path: None,
            // direction,
            age,
            core_traits,
            stable_traits,
            lifestyle,
            living_area,
            short_term_mem: ShortTerm::default(),
            spatial: SpatialMemory::default(),
            view_range,
            movement_cooldown_input,
            movement_cooldown: movement_cooldown_input
        }
    }
    pub fn ascend(&mut self, map: &WorldMap) {
        self.spatial = SpatialMemory::god(map);
    }
    pub fn short_term_mem(&self) -> &ShortTerm {
        &self.short_term_mem
    }
    pub fn short_term_mem_mut(&mut self) -> &mut ShortTerm {
        &mut self.short_term_mem
    }
    pub fn spatial_mem(&self) -> &SpatialMemory {
        &self.spatial
    }
    pub fn spatial_mem_mut(&mut self) -> &mut SpatialMemory {
        &mut self.spatial
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
    pub async fn decide(&mut self, llama: &Ollama, datetime: &DateTime) {}

    pub fn get_location(&self, map: &WorldMap) -> (String, String) {
        map.get_position_info(&self.position)
            .unwrap_or(("Unknown".to_string(), "Unknown".to_string()))
    }
    pub async fn tick(&mut self, time: &crate::misc::time::Time) {
        //Check if current action is done
        //match action(type) {
        // MOVE => Move
        // TALK => Talk
        // Misc => wait
        //}
        //
        // let action_active = self.short_term_mem().curr_action.is_some();
        if let Some(action) = &self.short_term_mem().curr_action{
            if action.completed(time){

            }
            else {
                match action.description(){
                    val if val == "MOVE".to_string() => {
                        if self.movement_cooldown <= 0{
                            //Move
                            self._move();
                        }
                        else {
                            //Wait
                            self.movement_cooldown -= 1;
                        }
                    },
                    val if val == "TALK".to_string() => {
                        todo!()
                    },
                    _ => {

                    }
                }                
            }
        }
        else{
            //This should never happen but like ok

        }
        // match self.short_term_mem.curr_action.completed(time){
        //     true => {

        //     }
        //     false => {
        //         match self.short_term_mem().curr_action
        //     }
        // }
    }
    pub fn movement_cooldown_max(&self) -> i64{
        self.movement_cooldown_input
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
