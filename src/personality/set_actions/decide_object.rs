use std::{collections::VecDeque, error::Error};

use futures::future::ok;
use serde::de::IntoDeserializer;
use serde_json::{json, Value};

use crate::{
    misc::{
        ollama::{
            ollama::Ollama,
            options::{FormatPair, GenerateOptions},
        },
        time::DateTime,
    },
    personality::action::{Action, ProperAction},
    world::{navigation::Navigator, world_map::Coordinates},
    TEXT_MODEL, TIME_STEP,
};

impl crate::world::character::Character {
    //Also uses vague
    pub async fn decide_object(
        &mut self,
        llama: &Ollama,
        datetime: &DateTime,
        navigator: &Navigator,
    ) -> Result<String, Box<dyn Error>> {
        let objects = navigator.get_visible_objects(self);
        let prompt = self.pick_object(datetime, &objects.iter().map(|f| f.0).collect());
        let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), prompt);
        options.add_format_pair(
            "Container".to_string(),
            vec![FormatPair("object".to_string(), &json!("string"))],
        );
        if let Some(response_str) = llama.generate(options).await["response"].as_str() {
            // println!("{}", response_str);
            if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                if let Some(object) = response_json["object"].as_str() {
                    match object.to_string() {
                        val if val == "NONE".to_string() => {
                            return Ok("NONE".to_string());
                        }
                        object => {
                            let mut output = Ok(object.to_string());
                            if let Some(target) = &objects.get(&object) {
                                'outer: for pos in *target {
                                    for dx in -1isize..=1 {
                                        for dy in -1isize..=1 {
                                            if let (Ok(valid_x), Ok(valid_y)) = (
                                                TryInto::<usize>::try_into((pos.0 as isize + dx)),
                                                TryInto::<usize>::try_into((pos.1 as isize + dy)),
                                            ) {
                                                if let Some(valid_path) = navigator.get_path(
                                                    self.position().clone(),
                                                    Coordinates(valid_x, valid_y),
                                                ) {
                                                    // path = valid_path;
                                                    self.short_term_mem_mut().curr_action =
                                                        Some(Action::new(
                                                            (
                                                                "MOVING".to_string(),
                                                                "MOVING".to_string(),
                                                            ),
                                                            datetime.1,
                                                            valid_path.len() as i64
                                                                * (self.movement_cooldown_max()
                                                                    + 1)
                                                                * TIME_STEP,
                                                            ProperAction::MOVE.to_string(),
                                                            None,
                                                            None,
                                                            // Fn,
                                                        ));
                                                    self.set_path(valid_path);
                                                    return output;
                                                    // break 'outer;
                                                }
                                                return Err("No Valid Positions.".into());
                                            }
                                        }
                                    }
                                }
                            }
                            return Err("Object Not Found.".into());
                        }
                    }
                }
            }
            return Err(("Response Not Found.".into()));
        } else {
            Err("Prompt Generation Error.".into())
        }
        // Ok(())
    }
}
