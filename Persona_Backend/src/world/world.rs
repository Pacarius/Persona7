use futures::future::join_all;

use crate::misc::ollama::ollama::Ollama;
use crate::misc::time::{Date, DateTime, Time, DAY_LENGTH};
use crate::personality::action::Action;
use crate::TIME_STEP;

use super::character::Character;
use super::world_map::WorldMap;

// pub trait WorldListener {
//     fn tick(&self, time: &Time);
// }
pub struct World {
    map: WorldMap,
    pub datetime: DateTime,
}
impl World {
    pub fn new(map: WorldMap) -> Self {
        Self {
            map,
            datetime: DateTime(
                Date::new(1, crate::misc::time::Month::January),
                Time::from_hms((0, 0, 0)),
            ),
        }
    }
    pub fn get_map(&self) -> &WorldMap {
        &self.map
    }
    pub fn get_map_mut(&mut self) -> &mut WorldMap {
        &mut self.map
    }
    // pub async fn tick(&mut self) {
    //     self.datetime.1 + Time::from_hms((0, 0, 20));
    //     for ele in self.map.get_characters_mut() {
    //         ele.tick(&self.datetime.1);
    //     }
    // }
}
impl World {
    pub async fn day_start(&mut self, llama: &Ollama) -> Time {
        let date = self.datetime.0.clone();
        self.get_map_mut().day_start(llama, date).await;
        let mut day_start_time: Vec<&Time> = self
            .get_map()
            .get_characters()
            .iter()
            .map(|c| &c.short_term_mem().plan_vague.iter().nth(1).unwrap().start)
            .collect();
        day_start_time.sort();
        **day_start_time.iter().nth(0).unwrap()
    }
    pub async fn tick(&mut self, llama: &Ollama) -> bool{
        let new_datetime = self.datetime.clone() + Time::from_seconds(TIME_STEP);
        let o = self.get_map_mut().update(&new_datetime, llama).await;
        self.datetime = new_datetime;
        o
    }
    pub async fn set_day_end(&self) -> Time {
        let mut day_end_time: Vec<&Time> = self
            .get_map()
            .get_characters()
            .iter()
            .map(|c| &c.short_term_mem().plan_vague.iter().last().unwrap().start)
            .collect();
        day_end_time.sort();
        **day_end_time.iter().last().unwrap()
    }
    pub async fn day(&mut self, llama: &Ollama) {
        println!("{}", self.get_map());
        let start_time = self.day_start(llama).await;
        // let end_time = self.set_day_end().await;
        let end_time  =Time::from_seconds(DAY_LENGTH - 1);
        println!(
            "Day start/end time determined: {}; {}.",
            start_time, end_time
        );
        self.get_map().get_characters().iter().for_each(|c| {
            println!("{:?}", c.short_term_mem().plan_vague);
        });
        // let buffer = self.get_map().get_characters().iter().map(|c| c.short_term_mem().curr_action).collect::<Vec<Action>>().iter().filter()
        self.datetime.1 = start_time - Time::from_seconds(1);
        //Log invterval in steps
        let log_interval = 200;
        let mut log_cooldown = 0;
        while !self.tick(llama).await{
            // self.tick(llama).await;
            // self.get_map_mut().calculate_colliders();
            // if log {
            // println!("{}", log_cooldown);
            if log_cooldown >= log_interval {
                self.get_map().get_characters().iter().for_each(|c| {
                    println!("{:?}", c.short_term_mem().curr_action);
                    println!("{:?}", c.position());
                    // println!("{:?}", )
                    // println!("{:?}", c.short_term_mem().action_buffer);
                    // println!("{:?}", c.short_term_mem().plan_vague);
                });
                println!("{}", self.get_map());
                println!("{}", self.datetime);
                log_cooldown = 0;
            } else {
                log_cooldown += 1;
            }
        }
        
                println!("{}", self.get_map());
        // {
        //     // Manually ending day LOL
        //     // for c in self.get_map_mut().get_characters_mut() {}
        //     // for c in self.get_map_mut().get_characters_mut() {
        //     //     c.clear();
        //     // }
        // }
    }
}
