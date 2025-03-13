// use misc::time::{Month, Time};

use std::{collections::HashMap, path::Path};

use misc::{
    ollama::{
        self,
        ollama::Ollama,
        options::{FormatPair, GenerateOptions},
    },
    time::{Date, DateTime, Month, Time},
};
use serde_json::json;
use world::{
    // test::test_char,
    world_map::{Coordinates, WorldMap},
};
// use crate::world::helpers::MapHelper;
use xcf::Xcf;

// use sqlite::data::{DBData, DBDataMap, DBDataType};

mod misc;
mod personality;
mod world;
const TEXT_MODEL: &str = "llama3.2";
const EMBEDDING_MODEL: &str = "nomic-embed-text";

#[tokio::main]
async fn main() {
    // let xcf = Xcf::open("test/Sample.xcf").unwrap();
    // println!("{:?}", xcf.layers.iter().nth(0).unwrap().pixel(0, 0));
    // map::run();
    // let wm = MapHelper::new(Path::new("test/Sample.json"));
    // println!("{:?}", wm);
    let ollama = Ollama::new("localhost:11434".to_string(), true);
    // let test_generate = ollama.test_generate().await["response"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_generate);
    // let test_chat = ollama.test_chat().await["message"]["content"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_chat);
    // println!("{}", adapter.characters.values().nth(0).unwrap());
    // let adapter = Adapter::new(ollama);
    // adapter.initialise().await;
    // adapter.main().await;

    // println!("{}", DBDataMap{ 0: HashMap::from([(String::from("FUCK"), DBDataType::BLOB), (String::from("SHIT"), DBDataType::TEXT)]) });
    let mut world = world::test::test_world();
    world.day_start(&ollama).await;
    println!(
        "{:?}",
        world
            .get_map()
            .get_character("Man".to_string())
            .short_term_mem()
            .plan_vague
    );
    // println!("{}", world);
    // println!("{:?}", world.get_visible_colliders("Man".to_string(), 5));
    // let a = world.get_characters().iter().nth(0).unwrap().rest();
    // let mut test = GenerateOptions::new("llama3.2".to_string(), "Give me a random time in HH:MM:SS format.".to_string());
    // test.format(vec![FormatPair("time".to_string(), &json!("string"))]);
    // test.format = Some("{'time': {'type': 'string'}}".to_string());
    // let out = &ollama.generate(test).await["response"];
    // println!("{}", out);
    // let man = test_char();
    // println!("{:?}", world.get_character("Man".to_string()).day_start(&ollama).await);
    // println!("{}", world.get_path_visual("Man".to_string(), Coordinates((0, 16))));
    // let mut datetime = DateTime(Date::new(1, Month::January), Time::from_hms((2, 0, 0)));
    // datetime.add(Time::from_hms((23, 0, 0)));
    // loop{
    //     datetime.0.add_days(1);
    //     println!("{:?}", datetime);
    // }
    // let times = (Time::from_hms((23, 0, 0)), Time::from_hms((23, 0, 0)));
    // println!("{:?}", times.0 + times.1);
}
