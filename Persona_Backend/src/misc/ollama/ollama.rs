use std::error::Error;

use reqwest::Client;
use serde_json::{json, Value};

use crate::{OLLAMA_ENDPOINT, TEXT_MODEL};

use super::options::{self, ChatOptions, ChatRole, GenerateOptions, Message};

pub struct Ollama {
    endpoint: String,
    client: Client,
    logging: bool,
}
impl Ollama {
    pub fn new(logging: bool) -> Ollama {
        Ollama {
            endpoint: format!("http://{}/api/", OLLAMA_ENDPOINT),
            client: Client::new(),
            logging,
        }
    }
    pub async fn get_response<T: serde::Serialize>(
        &self,
        options: T,
        append: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let (client, endpoint) = (&self.client, format!("{}{}", &self.endpoint, append));
        // println!("{}", endpoint);
        if self.logging {
            println!("Posting {} to {}", json!(&options), endpoint)
        };
        let response = client.post(endpoint).json(&options).send().await;
        let response = &response?.text().await?;
        if self.logging {
            println!("{}", response)
        }
        let object: Value = serde_json::from_str(&response)?;
        Ok(object)
    }
    // async fn parse(result: Result<reqwest::Response, reqwest::Error>) -> Result<Vec<Value>, Box<dyn Error>>{
    //     result?.text().await?.split_inclusive('}').map(|s|{
    //         serde_json::from_str(s).or_else(|_| Ok(Value::Null))
    //     }).collect::<Result<Vec<_>, _>>()
    // }
    pub async fn chat(&self, options: options::ChatOptions) -> Value {
        self.get_response(options, "chat").await.unwrap()
    }
    pub async fn generate(&self, options: options::GenerateOptions) -> Value {
        self.get_response(options, "generate").await.unwrap()
    }
    pub async fn test_generate(&self) -> Value {
        self.generate(GenerateOptions::new(
            TEXT_MODEL.to_string(),
            "Why do I want to die".to_string(),
        ))
        .await
    }
    pub async fn test_chat(&self) -> Value {
        self.chat(ChatOptions::new(
            TEXT_MODEL.to_string(),
            vec![Message::new(
                ChatRole::USER.to_string(),
                "Why do I want to die".to_string(),
            )],
        ))
        .await
    }
    // pub async fn chat(&self, model: String, prompt: String) -> Result<Value, Box<dyn Error>>{

    // }

    // pub async fn test(&self) -> bool{
    //     let response = self.generate("llama3.2".to_string(), "Why is the sky blue?".to_string()).await;
    //     response.is_some_and(|output|{
    //         println!("{}", output);
    //         true
    //     })
    // }
}
