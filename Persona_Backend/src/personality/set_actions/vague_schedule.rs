use std::error::Error;

use serde_json::{json, Value};

use crate::{
    misc::{
        ollama::{
            ollama::Ollama,
            options::{FormatPair, FormatTriple, GenerateOptions},
        },
        time::{Date, Time},
    },
    personality::action::ActionBare,
    TEXT_MODEL,
};

impl crate::world::character::Character {
    pub async fn daily_schedule(
        &mut self,
        llama: &Ollama,
        date: &Date,
    ) -> Result<(), Box<dyn Error>> {
        let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), self.vague(date));
        options.add_format_triple(
            "Schedule".to_string(),
            FormatTriple(
                "actions".to_string(),
                vec![
                    FormatPair("action_description".to_string(), &json!("string")),
                    FormatPair("start_time".to_string(), &json!("string")),
                    FormatPair("end_time".to_string(), &json!("string")),
                ],
            ),
        );

        if let Some(response_str) = llama.generate(options).await["response"].as_str() {
            // println!("{}", response_str);
            if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                let mut plans = vec![];

                if let Some(schedule) = response_json["Schedule"].as_array() {
                    for plan in schedule {
                        if let (Some(action_desc), Some(start_time), Some(end_time)) = (
                            plan["action_description"].as_str(),
                            plan["start_time"].as_str(),
                            plan["end_time"].as_str(),
                        ) {
                            if let (Some(start), Some(end)) = (
                                Time::parse_time_pair(start_time),
                                Time::parse_time_pair(end_time),
                            ) {
                                let action_bare =
                                    ActionBare::new(action_desc.to_string(), start.0, end.0);
                                plans.push(action_bare);
                            }
                        }
                    }
                }

                let short_term_mem_mut = self.short_term_mem_mut();
                // Assuming the first and last entries in plan_vague are wake and sleep actions
                if let Some(wake_action) = short_term_mem_mut.plan_vague.first().cloned() {
                    if let Some(sleep_action) = short_term_mem_mut.plan_vague.last().cloned() {
                        short_term_mem_mut.plan_vague.clear();
                        short_term_mem_mut.plan_vague.push(wake_action);
                        short_term_mem_mut.plan_vague.extend(plans);
                        short_term_mem_mut.plan_vague.push(sleep_action);
                        return Ok(());
                    } else {
                        Err("Schedule Parsing Error.".into())
                    }
                } else {
                    Err("Schedule Parsing Error.".into())
                }
            } else {
                Err("Response Format Error.".into())
            }
        } else {
            Err("Response Format Error.".into())
        }
        // Err("Failed Vague Schedule".into())
    }
}
