use futures::future::join_all;

use crate::misc::ollama::ollama::Ollama;
use crate::misc::time::{Date, DateTime, Time};
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
    pub async fn day_start(&mut self, llama: &Ollama) {
        let date = self.datetime.0.clone();
        self.get_map_mut().day_start(llama, date).await;
    }
    pub async fn tick(&mut self) {
        let (new_time, _) = self.datetime.1 + Time::from_seconds(TIME_STEP);
        self.datetime.1 = new_time;
        self.get_map_mut().update(new_time).await;
    }
}
