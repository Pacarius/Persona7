use std::collections::HashMap;
use std::{fs, io};

use chat::adapter::Adapter;
use ollama::ollama::Ollama;
// use misc::time::{Month, Time};

mod personality;
mod misc;
mod ollama;
mod chat;
// mod sqlite;
#[tokio::main]
async fn main() {
    let ollama = Ollama::new("192.168.50.84:11434".to_string());
    let test_generate = ollama.test_generate().await["response"].as_str().unwrap().replace("\\n", "\n");
    let test_chat = ollama.test_chat().await["message"]["content"].as_str().unwrap().replace("\\n", "\n");
    // println!("{}", test_generate);
    println!("{}", test_chat);
    // Adapter::new();
}
