// use misc::time::{Month, Time};

use std::{collections::HashMap, path::Path};

use misc::{
    ollama::{
        self,
        ollama::Ollama,
        options::{FormatPair, FormatTriple, GenerateOptions},
    },
    time::{Date, DateTime, Month, Time},
};
use personality::action::fmt_abv;
use serde::Serialize;
use serde_json::{json, Value};
use world::{
    // test::test_char,
    navigation::Navigator,
    world_map::{Coordinates, WorldMap},
    worlds::yeong::yeong,
};
// use crate::world::helpers::MapHelper;
use xcf::Xcf;

// use sqlite::data::{DBData, DBDataMap, DBDataType};

mod misc;
mod personality;
mod world;
const TEXT_MODEL: &str = "llama3.2";
const EMBEDDING_MODEL: &str = "nomic-embed-text";
const TIME_STEP: i64 = 20;

#[tokio::main]
async fn main() {
    // let xcf = Xcf::open("test/Sample.xcf").unwrap();
    // println!("{:?}", xcf.layers.iter().nth(0).unwrap().pixel(0, 0));
    // map::run();
    // let wm = MapHelper::new(Path::new("test/Sample.json"));
    // println!("{:?}", wm);
    // let test_generate = ollama.test_generate().await["response"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_generate);
    // let test_chat = ollama.test_chat().await["message"]["content"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_chat);
    // println!("{}", adapter.characters.values().nth(0).unwrap());
    // let adapter = Adapter::new(ollama);
    // adapter.initialise().await;
    // adapter.main().await;

    // println!("{}", DBDataMap{ 0: HashMap::from([(String::from("FUCK"), DBDataType::BLOB), (String::from("SHIT"), DBDataType::TEXT)]) });
    let ollama = Ollama::new("192.168.50.84:11434".to_string(), false);
    // let ollama = Ollama::new("192.168.33.132:11434".to_string(), false);
    // let ollama = Ollama::new("localhost:11434".to_string(), false);
    let mut world = yeong();
    // println!("{}", world.get_map());
    world.day(&ollama).await;

    // world.day_start(&ollama).await;
    // let datetime = DateTime(Date::new(1, Month::January), Time::from_hms((10, 0, 0)));
    // let map = world.get_map_mut();
    // let navigator = Navigator::new(&map);
    // let character = map.get_character_mut("Ava Thompson".to_string());
    // character
    //     .decide_room(&ollama, &datetime, &navigator)
    //     .await
    //     .unwrap();
    // println!("{:?}", character.path());
    // println!(
    //     "{}",
    //     character.short_term_mem().curr_action.as_ref().unwrap()
    // );
    // character.decompose_task(&ollama, &datetime).await.unwrap();
    // for a in &character.short_term_mem().action_buffer {
    //     println!("{:?}", a);
    // }

    // let mut options = GenerateOptions::new(
    //     TEXT_MODEL.to_string(),
    //     world
    //         .get_map()
    //         .get_character("Ava Thompson".to_string())
    //         .rest_wake(),
    // );
    // options.add_format_triple("container2".to_string(), FormatTriple("data1".to_string(), vec![FormatPair("item1".to_string(), "string"), FormatPair("item2".to_string(), "string")]));
    // options.add_format_pair("container1".to_string(), vec![FormatPair("data1".to_string(), &json!("string")), FormatPair("data2".to_string(), &json!("string"))]);
    // options.add_format_pair(
    //     "Wake_Sleep".to_string(),
    //     vec![
    //         FormatPair("wake_time".to_string(), &json!("string")),
    //         FormatPair("sleep_time".to_string(), &json!("string")),
    //     ],
    // );
    // let options_value = serde_json::to_value(&options).unwrap();
    // println!("{}", options_value.to_string());
    // println!("{:?}", world.get_map().get_visible_objects(world.get_map().get_character("Ava Thompson".to_string())));
    // println!("{}", world.get_map());
    // world
    //     .get_map_mut()
    //     .set_path("Ava Thompson".to_string(), Coordinates(8, 33));
    // println!(
    //     "{}",
    //     world.get_map().get_path_visual("Ava Thompson".to_string())
    // );
    // loop {
    //     let set = world.get_map_mut().move_characters();
    //     if set.is_empty() {
    //         break;
    //     }
    //     println!("{:?}", set);
    //     println!(
    //         "{:?}",
    //         world
    //             .get_map()
    //             .get_visible_objects(world.get_map().get_character("Ava Thompson".to_string()))
    //     );
    // }

    // {
    //     let path = world
    //         .get_map()
    //         .get_path("Man".to_string(), Coordinates((0, 16)))
    //         .unwrap();
    //     world
    //         .get_map_mut()
    //         .get_character_mut("Man".to_string())
    //         .set_path(path);
    // }
    // while let Some(c) = world
    //     .get_map_mut()
    //     .get_character_mut("Man".to_string())
    //     ._move()
    // {
    //     println!("{}, {}", c.0, c.1)
    // }
    // println!("{}", world.get_map().get_path_visual("Man".to_string(), Coordinates((0, 16))));
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
    // let mut datetime = DateTime(Date::new(1, Month::January), Time::from_hms((2, 0, 0)));
    // datetime.add(Time::from_hms((23, 0, 0)));
    // loop{
    //     datetime.0.add_days(1);
    //     println!("{:?}", datetime);
    // }
    // let times = (Time::from_hms((23, 0, 0)), Time::from_hms((23, 0, 0)));
    // println!("{:?}", times.0 + times.1);
}
