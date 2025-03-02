//Temporary adapter for stanford personality storage
use indexmap::IndexMap;
use serde::Deserialize;
use std::{
    fmt::Display,
    fs::{self, exists, read_dir},
    path,
};

use crate::misc::ollama::{
    ollama::Ollama,
    options::{ChatOptions, ChatRole, Message},
};

#[derive(Deserialize, Debug)]
pub struct Scratch {
    daily_plan_req: String,
    name: String,
    first_name: String,
    last_name: String,
    age: u8,
    innate: String,
    learned: String,
    currently: String,
    lifestyle: String,
    living_area: String,
    act_event: Vec<Option<String>>,
}
impl Display for Scratch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            You will be answering as {}. You are {}.
            You are {}.
            {}
            {}
            {}
            You live in {}.
            You shall make up any details as required.
        ",
            self.name,
            self.age,
            self.innate,
            self.learned,
            self.currently,
            self.lifestyle,
            self.living_area,
        )
    }
}
pub struct Adapter {
    pub characters: IndexMap<String, Scratch>,
    client: Box<Ollama>,
}
impl Adapter {
    pub fn new(client: Ollama) -> Adapter {
        let mut characters = IndexMap::new();
        if exists("./external/").is_ok() {
            let targets = read_dir("./external/personas").unwrap();
            for target in targets {
                let tmp = target.unwrap();
                let scratch_path = tmp
                    .path()
                    .join(path::Path::new("bootstrap_memory/scratch.json"));
                // println!("{:?}", scratch_path);
                let content = fs::read_to_string(&scratch_path).unwrap();
                characters.insert(
                    tmp.file_name().into_string().unwrap(),
                    serde_json::from_str(&content).unwrap(),
                );
            }
        }
        // println!("{:?}", characters);
        Adapter {
            characters,
            client: Box::new(client),
        }
    }
    pub async fn initialise(&self) -> Vec<String> {
        let characters = &self.characters;
        println!(
            "{}",
            characters
                .iter()
                .enumerate()
                .map(|(i, item)| { format!("{}: {}\n", i, item.0) })
                .collect::<String>()
        );
        let target = characters.iter().nth(text_io::read!("{}")).unwrap();
        vec![String::from(format!("{}", target.1))]
    }
    pub async fn main(&self) {
        let model = "llama3.2";
        let mut chat_log = self.initialise().await;
        // println!("{}", chat_log);
        loop {
            let sending: String = text_io::read!("{}\n");
            chat_log.push(sending);
            let response = self
                .client
                .chat(ChatOptions::new(
                    model.to_string(),
                    vec![Message::new(
                        ChatRole::USER.to_string(),
                        chat_log.join("\n"),
                    )],
                ))
                .await;
            let response = response["message"]["content"]
                .as_str()
                .unwrap()
                .replace("\\n", "\n");
            println!("{}", response);
            chat_log.push(response);
        }
    }
    // pub fn send(input: String, client: crate::ollama::Ollama) -> Option<String>{
    //     let _ = client;
    //     // client.generate("llama3.2", )
    //     Some("".to_string())
    // }
}
