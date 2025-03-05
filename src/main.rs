// use misc::time::{Month, Time};

use std::{collections::HashMap, path::Path};

use misc::ollama::ollama::Ollama;
use world::world_map::{Coordinates, WorldMap};
// use crate::world::helpers::MapHelper;
use xcf::Xcf;

// use sqlite::data::{DBData, DBDataMap, DBDataType};

mod misc;
mod personality;
mod world;
// #[tokio::main]
fn main() {
    // let xcf = Xcf::open("test/Sample.xcf").unwrap();
    // println!("{:?}", xcf.layers.iter().nth(0).unwrap().pixel(0, 0));
    // map::run();
    // let wm = MapHelper::new(Path::new("test/Sample.json"));
    // println!("{:?}", wm);
    // let ollama = Ollama::new("localhost:11434".to_string());
    // let test_generate = ollama.test_generate().await["response"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_generate);
    // let test_chat = ollama.test_chat().await["message"]["content"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_chat);
    // println!("{}", adapter.characters.values().nth(0).unwrap());
    // let adapter = Adapter::new(ollama);
    // adapter.initialise().await;
    // adapter.main().await;

    // println!("{}", DBDataMap{ 0: HashMap::from([(String::from("FUCK"), DBDataType::BLOB), (String::from("SHIT"), DBDataType::TEXT)]) });
    let world = world::worlds::test_world();
    println!("{}", world);
    // println!("{:?}", world.get_visible_colliders("Man".to_string(), 5));
    // println!("{}", world.get_path_visual("Man".to_string(), Coordinates((0, 16))));
}
