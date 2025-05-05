use std::collections::VecDeque;

use crate::{
    misc::time::{DateTime, Time},
    personality::action::{Action, ActionBare, ActionEntry},
    world::{utils::MapObject, world_map::Coordinates},
};
#[derive(Debug, Clone)]
pub struct Path {
    source: Coordinates,
    path: VecDeque<Coordinates>,
    target: Coordinates,
}
impl Path {
    pub fn move_next(&mut self) {
        self.path.remove(0);
    }
}

#[derive(Debug, Clone)]
pub struct ShortTerm {
    //Daily
    pub goals: Vec<String>,
    pub plan_vague: Vec<ActionBare>,
    // pub plan_detailed: Vec<ActionBare>,
    pub action_buffer: VecDeque<ActionBare>,
    curr_action: Option<Action>,
    pub curr_object: Option<String>,
    //Constant. Path only contains something when character is actively moving.
    // pub path: Option<Path>,
    chat_target_buffer: Vec<String>,
}
impl ShortTerm {
    pub fn set_action(
        &mut self,
        action: Option<Action>,
        action_type: Option<String>,
        character_name: String,
    ) -> Option<ActionEntry> {
        // self.curr_action = action;
        let entry = match action {
            Some(a) => {
                self.curr_action = Some(a.clone());
                // ActionEntry::new("ACTION".into(), character_name, a., intended_duration, description, object, location)
                // Some(ActionEntry(a, character_name))
                Some(ActionEntry::new(
                    character_name,
                    a.description(),
                    self.curr_object.clone(),
                    action_type,
                ))
            }
            None => {
                self.clear_action();
                None
            }
        };
        entry
    }
    pub fn get_action(&self) -> Option<&Action> {
        self.curr_action.as_ref()
    }
    pub fn clear_action(&mut self) {
        // println!("CLEARING ACTION");
        self.curr_action = None;
    }
    pub fn surrounding_tasks(&self, time: Time) -> &[ActionBare] {
        let mut start_index = None;
        let mut end_index = None;
        // println!("Getting Tasks Surrounding: {}", time);
        for (i, task) in self.plan_vague.iter().enumerate() {
            if task.start <= time && task.end > time {
                start_index = Some(if i > 0 { i - 1 } else { i });
                end_index = Some(if i < self.plan_vague.len() - 1 {
                    i + 2
                } else {
                    i + 1
                });
                break;
            }
        }
        if let (Some(start), Some(end)) = (start_index, end_index) {
            let output = &self.plan_vague[start..end.min(self.plan_vague.len())];
            // for a in output{
            //     println!("Got: {}", a);
            // }
            output
        } else {
            &[]
        }
    }
}
impl Default for ShortTerm {
    fn default() -> Self {
        Self {
            goals: vec![],
            plan_vague: vec![],
            action_buffer: VecDeque::new(),
            curr_action: None,
            // path: None,
            chat_target_buffer: vec![],
            curr_object: None,
        }
    }
}
