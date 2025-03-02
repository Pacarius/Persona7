
// use misc::time::{Month, Time};

use std::collections::HashMap;

// use sqlite::data::{DBData, DBDataMap, DBDataType};

mod chat;
mod misc;
mod ollama;
mod personality;
mod sqlite;
#[tokio::main]
async fn main() {
    // let test_generate = ollama.test_generate().await["response"].as_str().unwrap().replace("\\n", "\n");
    // let test_chat = ollama.test_chat().await["message"]["content"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_generate);
    // println!("{}", test_chat);
    // println!("{}", adapter.characters.values().nth(0).unwrap());
    // let adapter = Adapter::new(ollama);
    // adapter.initialise().await;
    // adapter.main().await;

    // let ollama = Ollama::new("localhost:11434".to_string());
    // println!("{}", DBDataMap{ 0: HashMap::from([(String::from("FUCK"), DBDataType::BLOB), (String::from("SHIT"), DBDataType::TEXT)]) });
}
