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
        action::{Action, ActionBare, ProperAction},
        memory::{short_term::ShortTerm, spatial::SpatialMemory},
    },
    TEXT_MODEL,
};

use super::{
    navigation::{self, Navigator},
    world_map::{Coordinates, Room, WorldMap},
};

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

#[derive(Debug)]
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
    location: Option<(String, String)>,
    path: Option<VecDeque<Coordinates>>,
    view_range: i64,
    movement_cooldown_input: i64, //Extra number for inputting update to movement ratio.
    movement_cooldown: i64,
    state_controller: Decision, // moved_this_turn: bool,
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
        movement_cooldown_input: i64,
    ) -> Self {
        Character {
            name,
            sprite,
            position,
            location: None,
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
            movement_cooldown: movement_cooldown_input,
            state_controller: Decision::ROOM, // moved_this_turn: false,
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
    pub fn action_buffer_mut(&mut self) -> &mut VecDeque<ActionBare> {
        &mut self.short_term_mem_mut().action_buffer
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
    // pub async fn decide(&mut self, llama: &Ollama, datetime: &DateTime) {}

    // pub fn get_location(&self, map: &WorldMap) -> (String, String) {
    //     map.get_position_info(&self.position)
    //         .unwrap_or(("Unknown".to_string(), "Unknown".to_string()))
    // }
    pub async fn tick(
        &mut self,
        datetime: &DateTime,
        navigator: &Navigator,
        llama: &Ollama,
    ) -> Option<(String, String)> {
        println!("Tick started for character: {}", self.name);
        println!("Current state: {:?}", self.state_controller);

        let mut output = None;

        self.state_controller = match &self.state_controller {
            Decision::ROOM => {
                // if let Some(curr_action) = &mut self.short_term_mem().curr_action{
                //     if curr_action.description() == "MOVE".to_string(){
                //         if self._move().is_none(){
                //             self.short_term_mem_mut().curr_action = None;
                //             Decision::OBJECT
                //         }
                //     } else {
                //     }
                //     Decision::ROOM
                // }
                // Decision::OBJECT
                match &self.short_term_mem().curr_action {
                    None => {
                        println!("State: ROOM - Deciding room...");
                        if let Err(e) = self.decide_room(llama, datetime, navigator).await {
                            println!("Error while deciding room: {:?}", e);
                        }
                        println!("Room decision completed.");
                        Decision::ROOM
                    }
                    Some(a) => {
                        if a.description() == "MOVE".to_string() && self._move().is_some() {
                            Decision::ROOM
                        } else {
                            self.short_term_mem_mut().curr_action = None;
                            Decision::OBJECT
                        }
                    }
                }
            }
            Decision::OBJECT => match &self.short_term_mem().curr_action {
                None => {
                    println!("State: OBJECT - Deciding object...");
                    match self.decide_object(llama, datetime, navigator).await {
                        Ok(target) => {
                            println!("Object decision successful. Target: {}", target);
                            output = Some((self.name.clone(), target));
                        }
                        Err(e) => {
                            println!("Error while deciding object: {:?}", e);
                        }
                    }
                    Decision::OBJECT
                }
                Some(a) => {
                    if a.description() == "MOVE".to_string() && self._move().is_some() {
                        Decision::OBJECT
                    } else {
                        self.short_term_mem_mut().curr_action = None;
                        Decision::DECOMPOSE
                    }
                }
            },
            Decision::DECOMPOSE => {
                println!("State: DECOMPOSE - Decomposing task...");
                if let Err(e) = self.decompose_task(llama, datetime).await {
                    println!("Error while decomposing task: {:?}", e);
                }
                println!("Task decomposition completed.");
                Decision::ACT
            }
            Decision::ACT => {
                println!("State: ACT - Performing action...");
                if let Some(task) = self
                    .short_term_mem()
                    .surrounding_tasks(datetime.1)
                    .iter()
                    .nth(1)
                {
                    if task.end < datetime.1 {
                        println!("Current task has ended. Checking current action...");
                        if let Some(action) = &self.short_term_mem().curr_action {
                            if action.completed(&datetime.1) {
                                println!("Action completed. Fetching next action from buffer...");
                                let action_buffer = self.action_buffer_mut();
                                if let Some(new_action) = action_buffer.pop_front() {
                                    println!("New action found: {:?}", new_action);
                                    let current_object = match &self.short_term_mem().curr_object {
                                        Some(o) => Some(o.name()),
                                        None => None,
                                    };
                                    self.short_term_mem_mut().curr_action = Some(Action::new(
                                        navigator.get_position_info(&self.position).unwrap(),
                                        new_action.start,
                                        (new_action.end - new_action.start).in_seconds(),
                                        new_action.description,
                                        current_object.cloned(),
                                        None,
                                    ));
                                    println!("New action set.");
                                } else {
                                    println!("No new action found in buffer.");
                                }
                            } else {
                                println!(
                                    "Action still in progress. Description: {}",
                                    action.description()
                                );
                                if action.description() == "TALK".to_string() {
                                    println!("Executing TALK action...");
                                    todo!()
                                } else {
                                    println!(
                                        "Unknown action description: {}",
                                        action.description()
                                    );
                                }
                            }
                        }
                        Decision::ROOM
                    } else {
                        println!("Task is still ongoing. Staying in ACT state.");
                        Decision::ACT
                    }
                } else {
                    println!("No surrounding tasks found. Returning to ROOM state.");
                    Decision::ROOM
                }
            }
        };

        println!("Tick completed for character: {}", self.name);
        println!("Next state: {:?}", self.state_controller);
        output
    }
    pub fn movement_cooldown_max(&self) -> i64 {
        self.movement_cooldown_input
    }
    pub fn clear(&mut self) {
        self.short_term_mem_mut().action_buffer.clear();
        self.short_term_mem_mut().plan_vague.clear();
        self.short_term_mem_mut().path = None;
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

#[derive(Debug)]
enum Decision {
    ROOM,
    OBJECT,
    DECOMPOSE,
    ACT,
}
