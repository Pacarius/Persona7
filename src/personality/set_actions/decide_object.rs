use std::{char, collections::VecDeque, error::Error};

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
    world::{
        character::{self, Character},
        navigation::Navigator,
        world_map::Coordinates,
    },
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
        let prompt = self.pick_object(datetime, &objects.iter().map(|f| f.0).collect())?;
        let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), prompt);
        options.add_format_pair(
            "Container".to_string(),
            vec![FormatPair("object".to_string(), &json!("string"))],
        );
        if let Some(response_str) = llama.generate(options).await["response"].as_str() {
            // println!("{}", response_str);
            if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                if let Some(object) = response_json["Container"]["object"].as_str() {
                    // println!("{}", response_str);
                    match object.to_string() {
                        val if val == "NONE".to_string() => {
                            return Ok("NONE".to_string());
                        }
                        object => {
                            let mut output = Ok(object.to_string());
                            if let Some(target) = &objects.get(&object) {
                                println!("target: {}({:?})", object, target);
                                if target.0 {
                                    'outer: for pos in &target.1 {
                                        for dx in -1..=1 {
                                            for dy in -1..=1 {
                                                // Skip the object's own position (dx = 0, dy = 0)
                                                if dx == 0 && dy == 0 {
                                                    continue;
                                                }

                                                if let (Ok(valid_x), Ok(valid_y)) = (
                                                    TryInto::<usize>::try_into(pos.0 as isize + dx),
                                                    TryInto::<usize>::try_into(pos.1 as isize + dy),
                                                ) {
                                                    Self::decide_object_ex(
                                                        self, navigator, valid_x, valid_y, datetime,
                                                    );
                                                    return output;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    for pos in &target.1 {
                                        Self::decide_object_ex(
                                            self, navigator, pos.0, pos.1, datetime,
                                        );
                                    }
                                }
                            }
                            return Err("No Valid Positions.".into());
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
    fn decide_object_ex(
        character: &mut Character,
        navigator: &Navigator,
        valid_x: usize,
        valid_y: usize,
        datetime: &DateTime,
    ) {
        if let Some(valid_path) =
            navigator.get_path(character.position().clone(), Coordinates(valid_x, valid_y))
        {
            // Found a valid path
            character.short_term_mem_mut().curr_action = Some(Action::new(
                ("MOVING".to_string(), "MOVING".to_string()),
                datetime.1,
                valid_path.len() as i64 * (character.movement_cooldown_max() + 1) * TIME_STEP,
                ProperAction::MOVE.to_string(),
                None,
                None,
            ));
            character.set_path(valid_path);
        }
    }
}
