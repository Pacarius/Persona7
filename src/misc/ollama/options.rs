use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Serialize)]
pub struct ModelFile {
    mirostat: Option<i64>,
    mirostat_eta: Option<f32>,
    mirostat_tau: Option<f32>,
    num_ctx: Option<i64>,
    repeat_last_n: Option<i64>,
    repeat_penalty: Option<f32>,
    temperature: Option<f32>,
    seed: Option<i64>,
    stop: Option<String>,
    num_predict: Option<i64>,
    top_k: Option<i64>,
    top_p: Option<f32>,
    min_p: Option<f32>,
}

impl ModelFile {
    pub fn new() -> Self {
        Self {
            mirostat: None,
            mirostat_eta: None,
            mirostat_tau: None,
            num_ctx: None,
            repeat_last_n: None,
            repeat_penalty: None,
            temperature: None,
            seed: None,
            stop: None,
            num_predict: None,
            top_k: None,
            top_p: None,
            min_p: None,
        }
    }
}
#[derive(Serialize)]
pub struct GenerateOptions {
    model: String,
    prompt: String,
    suffix: Option<String>,
    images: Option<Vec<String>>,
    pub format: Option<Value>,
    options: Option<ModelFile>,
    system: Option<String>,
    template: Option<String>,
    stream: Option<bool>,
    raw: Option<bool>,
    keep_alive: Option<String>,
    context: Option<String>,
}
pub struct FormatPair<T: Serialize>(pub String, pub T);
pub struct FormatTriple<T: Serialize>(pub String, pub Vec<FormatPair<T>>);
impl GenerateOptions {
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            suffix: None,
            images: None,
            format: None,
            options: None,
            system: None,
            template: None,
            stream: Some(false),
            raw: None,
            keep_alive: None,
            context: None,
        }
    }
    pub fn format_pair(&mut self, targets: Vec<FormatPair<&impl Serialize>>) {
        let mut properties = Map::new();
        let mut required = Vec::new();

        for FormatPair(name, value) in targets {
            properties.insert(name.clone(), json!({"type": value}));
            required.push(name);
        }

        let json = json!({
            "type": "object",
            "properties": properties,
            "required": required,
        });
        self.format = Some(json);
        // println!("{}", json.to_string());
    }
    pub fn format_triple<T: Serialize>(&mut self, source: FormatTriple<T>) {
        let mut properties = serde_json::Map::new();
        for pair in source.1 {
            properties.insert(pair.0, json!({"type": pair.1}));
        }

        let json = json!({
            "type": "object",
            "properties": {
                source.0.clone(): {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": properties,
                        "required": properties.keys().collect::<Vec<&String>>()
                    }
                }
            },
            "required": [source.0]
        });
        self.format = Some(json);
    }
}
#[derive(Serialize)]
pub struct Message {
    role: String,
    content: String,
    images: Option<Vec<String>>,
    tool_calls: Option<Vec<String>>,
}

impl Message {
    pub fn new(role: String, content: String) -> Self {
        Self {
            role,
            content,
            images: None,
            tool_calls: None,
        }
    }
}
#[derive(Serialize)]
pub struct ChatOptions {
    model: String,
    messages: Vec<Message>,
    tools: Option<Vec<String>>,
    format: Option<String>,
    options: Option<ModelFile>,
    stream: Option<bool>,
    keep_alive: Option<String>,
}

impl ChatOptions {
    pub fn new(model: String, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            tools: None,
            format: None,
            options: None,
            stream: Some(false),
            keep_alive: None,
        }
    }
}
pub enum ChatRole {
    SYSTEM,
    USER,
    ASSISTANT,
    TOOL,
}
impl Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChatRole::SYSTEM => "system",
                ChatRole::USER => "user",
                ChatRole::ASSISTANT => "assistant",
                ChatRole::TOOL => "tool",
            }
        )
    }
}
