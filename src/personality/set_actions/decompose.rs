use std::error::Error;

use serde_json::{json, Value};

use crate::{
    misc::{
        ollama::{
            ollama::Ollama,
            options::{FormatPair, FormatTriple, GenerateOptions},
        },
        time::{DateTime, Time},
    },
    personality::action::ActionBare,
    TEXT_MODEL,
};

impl crate::world::character::Character {
    pub async fn decompose_task(
        &mut self,
        llama: &Ollama,
        datetime: &DateTime,
    ) -> Result<(), Box<dyn Error>> {
        //Called when new action starts.
        // println!("{}", self.decompose(datetime));
        if let Ok((prompt, total_duration, start_time)) = self.decompose(datetime) {
            let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), prompt);
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
                if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                    if let Some(tasks) = response_json["Detailed_Tasks"].as_array() {
                        let mut accumulated = 0;
                        tasks.into_iter().for_each(|task| {
                            if let (Some(details), Some(duration)) = (
                                task["subtask_details"].as_str(),
                                task["subtask_duration"].as_i64(),
                            ) {
                                // Check if adding this subtask exceeds the total duration
                                if accumulated + duration <= total_duration {
                                    accumulated += duration;
                                    let duration = Time::from_seconds(duration * 60);
                                    let start_time =
                                        start_time + Time::from_seconds(accumulated * 60);
                                    self.action_buffer_mut().push_back(ActionBare::new(
                                        details.to_string(),
                                        start_time.0,
                                        (start_time.0 + duration).0,
                                    ));
                                } else {
                                    // Stop adding subtasks if the total duration is exceeded
                                    return;
                                }
                            }
                        });
                    }
                    //Remember this shit is all in minutes
                    // println!("Source: {} \n{}", self.decompose(datetime), response_str);
                    // if let Ok(re)
                    // Ok(())
                    // self.short_term_mem_mut().action_buffer
                    Ok(())
                    // todo!()
                } else {
                    Err("Ollama Response Error.".into())
                }
            } else {
                Err("Ollama Response Error.".into())
            }
        } else {
            Err("Decompose (Prompt) Error.".into())
        }
    }
}
