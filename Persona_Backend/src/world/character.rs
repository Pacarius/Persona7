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
        action::{Action, ActionBare, ActionEntry, ProperAction},
        memory::{short_term::ShortTerm, spatial::SpatialMemory},
    },
    TEXT_MODEL, TIME_STEP,
};

use super::{navigation::Navigator, utils::Room, world_map::Coordinates};

// use super::world::WorldListener;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Clone)]
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
    home_location: Option<(String, String)>,
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
            home_location: None,
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
            state_controller: Decision::WAKE, // moved_this_turn: false,
        }
    }
    pub fn ascend(&mut self, navigator: &Navigator) {
        self.spatial = SpatialMemory::god(navigator);
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
    pub fn _move(&mut self, timestamp: &Time) -> Option<((Coordinates, Coordinates), Action)> {
        if let Some(path) = &mut self.path {
            if path.len() >= 2 {
                let (from, to) = (path[0].clone(), path[1].clone());
                path.pop_front();
                //Add to database
                // todo!();

                self.position = to.clone();
                return Some((
                    (from.clone(), to.clone()),
                    Action::new(
                        self.location
                            .clone()
                            .unwrap_or(("NONE".into(), "NONE".into())),
                        timestamp.clone(),
                        TIME_STEP,
                        format!("{}|{}", from, to),
                        None,
                        None,
                    ),
                ));
            } else {
                path.clear();
            }
        }
        None
    }
    pub fn set_path(&mut self, path: VecDeque<Coordinates>) {
        self.path = Some(path)
    }
    //Characters ticking : New Day; Activity Ended; if Activity is moving from point A to point B.
    pub async fn day_start(&mut self, llama: &Ollama, date: &Date, navigator: &Navigator) {
        let name = self.name.clone();
        self.short_term_mem_mut().action_buffer = vec![].into();
        self.short_term_mem_mut().plan_vague = vec![];
        self.short_term_mem_mut().set_action(None, None, name);
        self.wake_time(llama).await;
        self.daily_schedule(llama, date).await;
        self.home_location = navigator.get_position_info(self.position());
        self.state_controller = Decision::WAKE;
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
    ) -> (bool, Option<ActionEntry>) {
        let mut entry = None;
        let name = self.name.clone();
        let curr_position = self.position().clone();
        let cd = self.movement_cooldown.clone();
        // println!("Tick started for character: {}", self.name);
        // println!("Current state: {:?}", self.state_controller);

        // let mut output = None;

        self.state_controller = match &self.state_controller {
            Decision::WAKE => match self.short_term_mem().get_action() {
                None => {
                    // println!()
                    let loc = match navigator.get_position_info(&self.position) {
                        Some(l) => l,
                        None => ("ERROR".to_string(), "ERROR".to_string()),
                    };
                    let name = self.name().clone();
                    let stm = self.short_term_mem_mut();
                    if let Some(action) = stm.surrounding_tasks(datetime.1).first() {
                        if action.end > datetime.1 {
                            entry = stm.set_action(
                                Some(Action::new(
                                    loc,
                                    datetime.1,
                                    (action.end - datetime.1).in_seconds(),
                                    ProperAction::SLEEP.to_string(),
                                    None,
                                    None,
                                )),
                                None,
                                name,
                            );
                        }
                    }
                    println!("State: WAKE - Setting current action to waking up...");
                    // self.short_term_mem_mut().curr_action = Action::new(navigator.get_position_info(), start_time, intended_duration, description, object, chat)
                    // if let Some(position) = navigator.get_position_info(&self.position){

                    // }
                    Decision::WAKE
                }
                Some(a) => {
                    let loc = match navigator.get_position_info(&self.position) {
                        Some(l) => l,
                        None => ("ERROR".to_string(), "ERROR".to_string()),
                    };
                    if a.completed(&datetime.1) {
                        if a.description() == ProperAction::SLEEP.to_string() {
                            // if a.completed(&datetime.1) {
                            entry = self.short_term_mem_mut().set_action(
                                Some(Action::new(
                                    loc,
                                    datetime.1,
                                    5 * 60,
                                    ProperAction::WAKE.to_string(),
                                    None,
                                    None,
                                )),
                                None,
                                name,
                            );
                            Decision::WAKE
                            // }
                        } else if a.description() == ProperAction::WAKE.to_string() {
                            // if a.completed(&datetime.1) {
                            self.short_term_mem_mut().clear_action();
                            Decision::ROOM
                            // }
                        } else {
                            Decision::WAKE
                        }
                    } else {
                        Decision::WAKE
                    }
                    // Decision::WAKE
                }
            },
            Decision::ROOM => match &mut self.short_term_mem().get_action() {
                None => {
                    println!("State: ROOM - Deciding room...");
                    match self.decide_room(llama, datetime, navigator).await {
                        Err(e) => {
                            println!("SOMETHING IS AMONG US");
                            Decision::OBJECT
                        }
                        Ok(e) => {
                            entry = Some(e);
                            Decision::ROOM
                        }
                    }
                    // println!("Room decision completed.");
                }
                Some(a) => {
                    let object = a.object().clone();
                    if a.description() == ProperAction::MOVE.to_string() {
                        if let Some(c_pair) = self._move(&datetime.1) {
                            entry = Some(ActionEntry::new(
                                name,
                                json!([c_pair.0 .0.to_string(), c_pair.0 .1.to_string()])
                                    .to_string(),
                                object,
                                Some(ProperAction::MOVE.to_string()),
                            ));
                            Decision::ROOM
                        } else {
                            self.short_term_mem_mut().clear_action();
                            Decision::OBJECT
                        }
                    } else {
                        self.short_term_mem_mut().clear_action();
                        Decision::OBJECT
                    }
                }
            },
            Decision::OBJECT => match &self.short_term_mem().get_action() {
                None => {
                    println!("State: OBJECT - Deciding object...");
                    match self.decide_object(llama, datetime, navigator).await {
                        Ok(target) => {
                            println!("Target decided: {:?}", target);
                            if target.0 == "NONE".to_string() {
                                self.short_term_mem_mut().curr_object = None;
                            } else {
                                self.short_term_mem_mut().curr_object = Some(target.0);
                            }
                            // Decision::DECOMPOSE
                        }
                        Err(e) => {}
                    }
                    Decision::DECOMPOSE
                }
                Some(a) => {
                    let object = a.object();
                    if a.description() == ProperAction::MOVE.to_string() {
                        if let Some(c_pair) = self._move(&datetime.1) {
                            entry = Some(ActionEntry::new(
                                name,
                                json!([c_pair.0 .0.to_string(), c_pair.0 .1.to_string()])
                                    .to_string(),
                                object,
                                Some(ProperAction::MOVE.to_string()),
                            ));
                            Decision::OBJECT
                        } else {
                            self.short_term_mem_mut().clear_action();
                            Decision::DECOMPOSE
                        }
                    } else {
                        self.short_term_mem_mut().clear_action();
                        Decision::DECOMPOSE
                    }
                }
            },
            Decision::DECOMPOSE => {
                println!("State: DECOMPOSE - Decomposing task...");
                // println!("Decomposing based on {}", datetime.1);
                if let Err(e) = self.decompose_task(llama, datetime).await {
                    // println!("Error while decomposing task: {:?}", e);
                }
                if let Some(decomposed_task) = self.short_term_mem().action_buffer.front().cloned()
                {
                    let current_object = match self.short_term_mem().curr_object.clone() {
                        Some(o) => Some(o),
                        None => None,
                    };
                    entry = self.short_term_mem_mut().set_action(
                        Some(Action::new(
                            navigator.get_position_info(&curr_position).unwrap(),
                            decomposed_task.start,
                            (decomposed_task.end - decomposed_task.start).in_seconds(),
                            decomposed_task.description.clone(),
                            current_object,
                            None,
                        )),
                        None,
                        name,
                    );
                    self.short_term_mem_mut().action_buffer.pop_front();
                }
                // println!("Task decomposition completed.");
                Decision::ACT
            }
            Decision::ACT => {
                if datetime.1
                    < self.short_term_mem().plan_vague.last().unwrap().start
                        - Time::from_seconds(10 * 60)
                {
                    let mut_mem = self.short_term_mem_mut();
                    if let Some(task) = mut_mem.surrounding_tasks(datetime.1).iter().nth(1) {
                        if let Some(action) = &mut_mem.get_action() {
                            if action.completed(&datetime.1) {
                                let action_buffer = self.action_buffer_mut();
                                if let Some(new_action) = action_buffer.pop_front() {
                                    let current_object =
                                        match self.short_term_mem().curr_object.clone() {
                                            Some(o) => Some(o),
                                            None => None,
                                        };
                                    entry = self.short_term_mem_mut().set_action(
                                        Some(Action::new(
                                            navigator.get_position_info(&curr_position).unwrap(),
                                            new_action.start,
                                            (new_action.end - new_action.start).in_seconds(),
                                            new_action.description,
                                            current_object,
                                            None,
                                        )),
                                        None,
                                        name,
                                    );
                                    Decision::ACT
                                } else {
                                    // No new action found, transition to ROOM state
                                    self.short_term_mem_mut().clear_action();
                                    Decision::ROOM
                                }
                            } else if action.description() == "TALK".to_string() {
                                todo!()
                            } else {
                                Decision::ACT
                            }
                        } else {
                            // curr_action is None, fallback to the current vague action
                            // let current_object = match &self.short_term_mem().curr_object {
                            //     Some(o) => Some(o),
                            //     None => None,
                            // };
                            let current_object: Option<&String> = None;
                            entry = mut_mem.set_action(
                                Some(Action::new(
                                    navigator.get_position_info(&curr_position).unwrap(),
                                    task.start,
                                    (task.end - task.start).in_seconds(),
                                    task.description.clone(),
                                    current_object.cloned(),
                                    None,
                                )),
                                None,
                                name,
                            );
                            Decision::ACT
                        }
                    } else {
                        // No surrounding tasks found, transition to ROOM state
                        Decision::ROOM
                    }
                } else {
                    Decision::GO_HOME
                }
            }
            Decision::GO_HOME => match &self.short_term_mem().get_action() {
                None => {
                    if let Some(target) = &self.home_location.clone() {
                        if let Some(position) = navigator.get_pos_room(target.clone()) {
                            if let Some(path) =
                                navigator.get_path(self.position().clone(), position)
                            {
                                entry = self.short_term_mem_mut().set_action(
                                    Some(Action::new(
                                        (
                                            navigator.get_position_info(&curr_position).unwrap().1,
                                            target.1.clone(),
                                        ),
                                        datetime.1,
                                        path.len() as i64 * (cd + 1) * TIME_STEP,
                                        ProperAction::MOVE.to_string(),
                                        None,
                                        None,
                                    )),
                                    None,
                                    name,
                                );
                                self.set_path(path);
                                Decision::GO_HOME
                            } else {
                                Decision::SLEEP
                                // SLEEP ON THE FLOOR
                            }
                            // Decision::SLEEP
                        } else {
                            Decision::SLEEP
                        }
                    } else {
                        Decision::SLEEP
                    }
                    // Decision::SLEEP
                }
                Some(a) => {
                    let object = a.object();
                    if let Some(c_pair) = self._move(&datetime.1) {
                        entry = Some(ActionEntry::new(
                            name,
                            // a.description(),
                            json!([c_pair.0 .0.to_string(), c_pair.0 .1.to_string()]).to_string(),
                            object,
                            Some(ProperAction::MOVE.to_string()),
                        ));
                        Decision::GO_HOME
                    } else {
                        self.short_term_mem_mut().clear_action();
                        Decision::GO_TO_BED
                    }
                }
            },
            Decision::GO_TO_BED => match &self.short_term_mem().get_action() {
                None => {
                    if let Some(bed_location) = navigator.get_visible_objects(&self).get("Bed") {
                        if let Some(path) = navigator.get_path(
                            self.position().clone(),
                            bed_location.1.first().unwrap().clone(),
                        ) {
                            entry = self.short_term_mem_mut().set_action(
                                Some(Action::new(
                                    navigator.get_position_info(&curr_position).unwrap(),
                                    datetime.1,
                                    path.len() as i64 * (cd + 1) * TIME_STEP,
                                    ProperAction::MOVE.to_string(),
                                    None,
                                    None,
                                )),
                                None,
                                name,
                            );
                            self.set_path(path);
                            Decision::GO_TO_BED
                        } else {
                            Decision::SLEEP
                        }
                    } else {
                        Decision::SLEEP
                    }
                }
                Some(a) => {
                    let object = a.object();
                    if let Some(c_pair) = self._move(&datetime.1) {
                        entry = Some(ActionEntry::new(
                            name,
                            json!([c_pair.0 .0.to_string(), c_pair.0 .1.to_string()]).to_string(),
                            object,
                            Some(ProperAction::MOVE.to_string()),
                        ));
                        Decision::GO_TO_BED
                    } else {
                        self.short_term_mem_mut().clear_action();
                        Decision::SLEEP
                    }
                }
            },
            Decision::SLEEP => match &self.short_term_mem().get_action() {
                None => {
                    let duration = Time::from_seconds(DAY_LENGTH - 1) - datetime.1;
                    // println!("Set action to sleeping. {}", duration);
                    entry = self.short_term_mem_mut().set_action(
                        Some(Action::new(
                            navigator.get_position_info(&curr_position).unwrap(),
                            datetime.1,
                            duration.in_seconds(),
                            ProperAction::SLEEP.to_string(),
                            None,
                            None,
                        )),
                        None,
                        name,
                    );
                    Decision::SLEEP
                }
                Some(a) => {
                    // println!("{:?}", a.completed(&datetime.1));
                    if a.completed(&datetime.1)
                        && a.description() == ProperAction::SLEEP.to_string()
                    {
                        self.short_term_mem_mut().clear_action();
                        Decision::WAKE
                    } else {
                        Decision::SLEEP
                    }
                }
            },
        };
        // println!("Tick completed for character: {}", self.name);
        // println!("Next state: {:?}", self.state_controller);
        // output
        let sleeping = if let Some(a) = &self.short_term_mem().get_action() {
            a.description() == ProperAction::SLEEP.to_string()
        } else {
            true
        };
        (sleeping, entry)
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

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
enum Decision {
    WAKE,
    ROOM,
    OBJECT,
    DECOMPOSE,
    ACT,
    GO_HOME,
    GO_TO_BED,
    SLEEP,
}
