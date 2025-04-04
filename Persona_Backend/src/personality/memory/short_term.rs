use std::collections::VecDeque;

use crate::{
    misc::time::Time,
    personality::action::{Action, ActionBare},
    world::world_map::Coordinates,
    world::utils::MapObject
};
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ShortTerm {
    //Daily
    pub goals: Vec<String>,
    pub plan_vague: Vec<ActionBare>,
    // pub plan_detailed: Vec<ActionBare>,
    pub action_buffer: VecDeque<ActionBare>,
    pub curr_action: Option<Action>,
    pub curr_object: Option<String>,
    //Constant. Path only contains something when character is actively moving.
    pub path: Option<Path>,
    chat_target_buffer: Vec<String>,
}
impl ShortTerm {
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
            path: None,
            chat_target_buffer: vec![],
            curr_object: None,
        }
    }
}
