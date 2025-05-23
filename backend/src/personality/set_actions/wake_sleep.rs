use serde_json::{json, Value};

use crate::{
    misc::{
        ollama::{
            ollama::Ollama,
            options::{FormatPair, GenerateOptions},
        },
        time::{Time, DAY_LENGTH},
    },
    personality::action::{ActionBare, ProperAction}, CONFIG,
    // TEXT_MODEL,
};

impl crate::world::character::Character {
    pub async fn wake_time(&mut self, llama: &Ollama) {
        let mut options = GenerateOptions::new(CONFIG.text_model.to_string(), self.rest_wake());
        options.add_format_pair(
            "Wake_Sleep".to_string(),
            vec![
                FormatPair("wake_time".to_string(), &json!("string")),
                FormatPair("sleep_time".to_string(), &json!("string")),
            ],
        );
        let short_term_mut = self.short_term_mem_mut();
        if let Some(response_str) = llama.generate(options).await["response"].as_str() {
            // println!("{}", response_str);
            if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                if let Some(wake_time_str) = response_json["Wake_Sleep"]["wake_time"].as_str() {
                    let wake_time = match Time::parse_time_pair(wake_time_str) {
                        Some(o) => o,
                        None => (Time::from_hms((6, 0, 0)), Time::from_seconds(0)),
                    };
                    let wake_action = ActionBare::new(
                        ProperAction::WAKE.to_string(),
                        Time::from_seconds(0),
                        wake_time.0,
                    );
                    short_term_mut.plan_vague.clear();
                    short_term_mut.plan_vague.push(wake_action);
                }
                if let Some(sleep_time_str) = response_json["Wake_Sleep"]["sleep_time"].as_str() {
                    let sleep_time = match Time::parse_time_pair(sleep_time_str) {
                        Some(o) => o,
                        None => (Time::from_hms((22, 0, 0)), Time::from_seconds(0)),
                    };
                    let sleep_action = ActionBare::new(
                        ProperAction::SLEEP.to_string(),
                        sleep_time.0,
                        Time::from_seconds(DAY_LENGTH - 1),
                    );
                    short_term_mut.plan_vague.push(sleep_action);
                }
            }
        }
    }
}
