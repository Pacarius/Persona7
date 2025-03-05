use std::path::Path;
use std::time::Duration;

use super::helpers::{Layer, MapHelper};
enum Directions{
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
pub struct Character {
    name: String,
    coordinates: (i32, i32),
    facing: Directions
}

pub struct WorldMap {
    game_map: MapHelper,
    world_time: Duration,
    characters: Vec<Character>,
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            game_map: MapHelper::new(Path::new("test/Sample.json")),
            world_time: Duration::from_secs(10), // 10 real life seconds = 1 in world minute
            characters: Vec::new(),
        }
    }
}