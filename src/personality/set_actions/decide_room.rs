use std::error::Error;

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
    world::{navigation::Navigator, world_map::WorldMap},
    TEXT_MODEL, TIME_STEP,
};

impl crate::world::character::Character {
    pub async fn decide_room(
        &mut self,
        llama: &Ollama,
        datetime: &DateTime,
        // map: &WorldMap,
        navigator: &Navigator,
    ) -> Result<(), Box<dyn Error>> {
        if let Ok(ro) = self.ro(
            datetime,
            navigator.get_position_info(self.position()).unwrap(),
        ) {
            let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), ro);
            options.add_format_pair(
                "location".to_string(),
                vec![
                    FormatPair("region".to_string(), &json!("string")),
                    FormatPair("room".to_string(), &json!("string")),
                ],
            );

            if let Some(response_str) = llama.generate(options).await["response"].as_str() {
                println!("{}", response_str);
                if let Ok(response_json) = serde_json::from_str::<Value>(response_str) {
                    if let (Some(region), Some(room)) = (
                        response_json["location"]["region"].as_str(),
                        response_json["location"]["room"].as_str(),
                    ) {
                        // let path =
                        //     map.set_path_character(self, (region.to_string(), room.to_string()))?;
                        if let Some(target) =
                            navigator.get_pos_room((region.to_string(), room.to_string()))
                        {
                            let path = navigator.get_path(self.position().clone(), target).unwrap();
                            self.short_term_mem_mut().curr_action = Some(Action::new(
                                ("MOVING".to_string(), "MOVING".to_string()),
                                datetime.1,
                                path.len() as i64 * (self.movement_cooldown_max() + 1) * TIME_STEP,
                                ProperAction::MOVE.to_string(),
                                None,
                                None,
                            ));
                            self.set_path(path);
                        };
                        Ok(())
                    } else {
                        Err(("Repsonse Format Error".into()))
                    }
                } else {
                    Err(("Response Not Found.".into()))
                }
            } else {
                Err(("Response Not Found.".into()))
            }
        } else {
            Err("Prompt Generation Error.".into())
        }
        // let mut options = GenerateOptions::new(TEXT_MODEL.to_string(), )
    }
}
