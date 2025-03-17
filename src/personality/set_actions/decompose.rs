use serde_json::json;

use crate::{
    misc::{
        ollama::{
            ollama::Ollama,
            options::{FormatPair, FormatTriple, GenerateOptions},
        },
        time::DateTime,
    },
    TEXT_MODEL,
};

impl crate::world::character::Character {
    pub async fn decompose_task(&mut self, llama: &Ollama, datetime: &DateTime) {
        //Called when new action starts.
        // println!("{}", self.decompose(datetime));
        let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), self.decompose(datetime));
        options.add_format_triple(
            "Detailed_Tasks".to_string(),
            FormatTriple(
                "Task".to_string(),
                vec![
                    FormatPair("subtask_details".to_string(), &json!("string")),
                    FormatPair("subtask_duration".to_string(), &json!("number")),
                    FormatPair("remaining_duration".to_string(), &json!("number")),
                ],
            ),
        );
        if let Some(response_str) = llama.generate(options).await["response"].as_str() {
            println!("{}", response_str);
        }
    }
}
