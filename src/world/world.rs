use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSlice;

use crate::misc::time::{Date, DateTime, Time};

use super::character::Character;
use super::world_map::WorldMap;

// pub trait WorldListener {
//     fn tick(&self, time: &Time);
// }
struct World {
    map: WorldMap,
    datetime: DateTime,
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
    pub async fn tick(&mut self) {
        self.datetime.1 + Time::from_hms((0, 0, 20));
        for ele in &self.map.get_characters() {
            ele.tick(&self.datetime.1);
        }
    }
}
