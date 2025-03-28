use std::fmt::Display;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
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
            seed: Some(4),
            stop: None,
            num_predict: None,
            top_k: None,
            top_p: None,
            min_p: None,
        }
    }
}
// #[derive(Serialize)]
pub struct GenerateOptions {
    model: String,
    prompt: String,
    suffix: Option<String>,
    images: Option<Vec<String>>,
    format: Map<String, Value>,
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
            format: Map::new(),
            options: None,
            system: None,
            template: None,
            stream: Some(false),
            raw: None,
            keep_alive: None,
            context: None,
        }
    }
    pub fn add_format_pair(
        &mut self,
        container: String,
        targets: Vec<FormatPair<&impl Serialize>>,
    ) {
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

        self.format.insert(container.clone(), json);
    }

    pub fn add_format_triple<T: Serialize>(&mut self, container: String, source: FormatTriple<T>) {
        let mut properties = serde_json::Map::new();
        for pair in source.1 {
            properties.insert(pair.0, json!({"type": pair.1}));
        }

        let json = json!({
            "type": "object",
            "properties": properties,
            "required": properties.keys().collect::<Vec<&String>>()
        });

        let container_json = json!({
            "type": "array",
            "items": json
        });

        self.format.insert(container.clone(), container_json);
    }

    // pub fn format(&self) -> Value {
    //     json!({
    //         "type": "object",
    //         "properties": self.format,
    //         "required": self.format.keys().collect::<Vec<&String>>(),
    //     })
    // }
}
impl Serialize for GenerateOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GenerateOptions", 12)?;
        state.serialize_field("model", &self.model)?;
        state.serialize_field("prompt", &self.prompt)?;
        state.serialize_field("suffix", &self.suffix)?;
        state.serialize_field("images", &self.images)?;
        state.serialize_field(
            "format",
            &json!({
                "type": "object",
                "properties": self.format,
                "required": self.format.keys().collect::<Vec<&String>>(),
            }),
        )?;
        state.serialize_field("options", &self.options)?;
        state.serialize_field("system", &self.system)?;
        state.serialize_field("template", &self.template)?;
        state.serialize_field("stream", &self.stream)?;
        state.serialize_field("raw", &self.raw)?;
        state.serialize_field("keep_alive", &self.keep_alive)?;
        state.serialize_field("context", &self.context)?;
        state.end()
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
