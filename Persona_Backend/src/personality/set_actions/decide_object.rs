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
    personality::action::{Action, ActionEntry, ProperAction},
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
    ) -> Result<(String, Option<ActionEntry>), Box<dyn Error>> {
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
                        val if val.to_uppercase() == "NONE".to_string() => {
                            return Ok(("NONE".to_string(), None));
                        }
                        object => {
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
                                                    if let Some(action_entry) =
                                                        Self::decide_object_ex(
                                                            self,
                                                            navigator,
                                                            valid_x,
                                                            valid_y,
                                                            datetime,
                                                            object.clone(),
                                                        )?
                                                    {
                                                        return Ok((
                                                            object.to_string(),
                                                            Some(action_entry),
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    for pos in &target.1 {
                                        if let Some(action_entry) = Self::decide_object_ex(
                                            self,
                                            navigator,
                                            pos.0,
                                            pos.1,
                                            datetime,
                                            object.clone(),
                                        )? {
                                            return Ok((object.to_string(), Some(action_entry)));
                                        }
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
        object: String,
    ) -> Result<Option<ActionEntry>, Box<dyn Error>> {
        if let Some(valid_path) =
            navigator.get_path(character.position().clone(), Coordinates(valid_x, valid_y))
        {
            let loc = match navigator.get_position_info(character.position()) {
                Some(l) => l,
                None => ("Error".to_string(), "Error".to_string()),
            };
            let cd = character.movement_cooldown_max().clone();
            let name = character.name().clone();
            // Found a valid path
            let action_entry = character.short_term_mem_mut().set_action(
                Some(Action::new(
                    loc,
                    datetime.1,
                    valid_path.len() as i64 * (cd + 1) * TIME_STEP,
                    ProperAction::MOVE.to_string(),
                    Some(object),
                    None,
                )),
                None,
                name,
            );
            character.set_path(valid_path);
            return Ok(action_entry);
        }
        Ok(None)
    }
}
