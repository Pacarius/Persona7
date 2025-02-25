use std::collections::HashMap;
use std::fmt::DebugMap;
use std::{fs, io};

use chat::adapter::Adapter;
use ollama::ollama::Ollama;
use text_io::read;
// use misc::time::{Month, Time};

mod personality;
mod misc;
mod ollama;
mod chat;
// mod sqlite;
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

    let ollama = Ollama::new("localhost:11434".to_string());

}
