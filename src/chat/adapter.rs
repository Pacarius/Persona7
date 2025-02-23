//Temporary adapter for stanford personality storage 
use std::{collections::HashMap, fs::{self, exists, read_dir, File}, path};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Scratch{
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
pub struct Adapter{
    characters: HashMap<String, Scratch>    
}
impl Adapter{
    pub fn new() -> Adapter{
        let mut characters = HashMap::new();
        if exists("./external/").is_ok(){
            let targets = read_dir("./external/personas").unwrap();
            for target in targets{
                let scratch_path = target
                .unwrap()
                .path()
                .join(path::Path::new("bootstrap_memory/scratch.json"));
            // println!("{:?}", scratch_path);
                let content = fs::read_to_string(&scratch_path).unwrap();
                characters.insert(
                    scratch_path.components().nth_back(1).unwrap().as_os_str().to_string_lossy().to_string(),
                     serde_json::from_str(&content).unwrap()
                );
            }
        }
        println!("{:?}", characters);
            Adapter{
                characters
        }
    }
    // pub fn send(input: String, client: crate::ollama::Ollama) -> Option<String>{
    //     let _ = client;
    //     // client.generate("llama3.2", )
    //     Some("".to_string())
    // }
}