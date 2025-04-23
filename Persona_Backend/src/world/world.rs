use futures::future::join_all;
use serde_json::{json, Value};
// use tokio::sync::broadcast::Sender;

use crate::misc::ollama::ollama::Ollama;
use crate::misc::time::{Date, DateTime, Time, DAY_LENGTH};
use crate::personality::action::Action;
use crate::server::message::{Message, MessageType};
use crate::TIME_STEP;

use super::character::Character;
use super::world_map::WorldMap;

// pub trait WorldListener {
//     fn tick(&self, time: &Time);
// }
pub struct World {
    map: WorldMap,
    pub datetime: DateTime,
    running: bool,
    first_day: bool, // event_queue: Vec<String>,
                     // tx: Option<tokio::sync::watch::Sender<Vec<String>>>
}
impl World {
    pub fn new(map: WorldMap) -> Self {
        Self {
            map,
            datetime: DateTime(
                Date::new(1, crate::misc::time::Month::January),
                Time::from_hms((0, 0, 0)),
            ),
            running: true,
            first_day: true, // event_queue: vec![],
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
    pub fn toggle_running(&mut self) {
        self.running = !self.running;
    }
    async fn day_start(&mut self, llama: &Ollama) -> Time {
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
    pub async fn tick(&mut self, llama: &Ollama, enable_logging: bool) -> Option<String> {
        let mut output = None;
        if !self.running {
            return output;
        }
        // let mut day_over = true;
        // if day_over{
        //     self.datetime.0.add_days(1);
        //     self.day_start(llama).await;
        // }

        let (new_datetime, day) = self.datetime.clone() + Time::from_seconds(TIME_STEP);
        if day || self.first_day {
            println!("Starting day...");
            self.day_start(llama).await;
            self.first_day = false;
        }
        // else{
        let day_logic_over = self.get_map_mut().update(&new_datetime, llama).await;
        if !day_logic_over.1.is_empty() {
            output = Some(
                Message::new(
                    MessageType::WEB,
                    format!("{:?}", day_logic_over.1),
                    Some(new_datetime.clone()),
                )
                .to_string(), // json![{
                              //         "type": "WEB",
                              //         "content": day_logic_over.1,
                              //         "timestamp": new_datetime.to_string()
                              // }]
                              // .to_string(),
            );
            println!("{:?}", output);
        }
        println!("{}", new_datetime.to_string());
        // todo!("Messaing Formatting; Forwarding To Clients");
        self.datetime = new_datetime;
        output
        // }
        // o
    }
    // pub async fn set_day_end(&self) -> Time {
    //     let mut day_end_time: Vec<&Time> = self
    //         .get_map()
    //         .get_characters()
    //         .iter()
    //         .map(|c| &c.short_term_mem().plan_vague.iter().last().unwrap().start)
    //         .collect();
    //     day_end_time.sort();
    //     **day_end_time.iter().last().unwrap()
    // }
    //Test Function
    // pub async fn day(&mut self, llama: &Ollama, enable_logging: bool) {
    //     let start_time = self.day_start(llama).await;
    //     self.datetime.1 = start_time - Time::from_seconds(1);

    //     let log_interval = 200;
    //     let mut log_cooldown = 0;

    //     while !self.tick(llama).await {
    //         if enable_logging && log_cooldown >= log_interval {
    //             self.get_map().get_characters().iter().for_each(|c| {
    //                 println!("{:?}", c.short_term_mem().curr_action);
    //                 println!("{:?}", c.position());
    //             });
    //             println!("{}", self.get_map());
    //             println!("{}", self.datetime);
    //             log_cooldown = 0;
    //         } else if enable_logging {
    //             log_cooldown += 1;
    //         }
    //     }

    //     if enable_logging {
    //         println!("{}", self.get_map());
    //     }
    // }
}
