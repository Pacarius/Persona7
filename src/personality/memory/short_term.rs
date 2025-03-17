use std::collections::VecDeque;

use crate::{
    misc::time::Time,
    personality::action::{Action, ActionBare},
    world::world_map::Coordinates,
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
// #[derive(Debug)]
pub struct ShortTerm {
    //Daily
    pub goals: Vec<String>,
    pub plan_vague: Vec<ActionBare>,
    pub plan_detailed: Vec<ActionBare>,
    //Constant. Path only contains something when character is actively moving.
    pub action: Action,
    pub path: Option<Path>,
}
impl ShortTerm {
    pub fn surrounding_tasks(&self, time: Time) -> &[ActionBare] {
        let mut start_index = None;
        let mut end_index = None;

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
            &self.plan_vague[start..end.min(self.plan_vague.len())]
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
            plan_detailed: vec![],
            action: Action::default(),
            path: None,
        }
    }
}
