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
// #[derive(Debug)]
// pub struct ShortTerm {
//     view_range: i64,
//     //Don't think this is relevant
//     concept_forget: i64,
//     reflection_time: i64,
//     reflection_size: i64,
//     overlap: i64,
//     kw_event_reflect: i64,
//     kw_thought_reflect: i64,
//     //
//     recency: i64,
//     relevancy: i64,
//     importance: i64,
//     recency_decay: f32,
//     importance_trig_max: i64,
//     importance_trig_curr: i64,
//     importance_ele: i64,
//     thought_count: i64,
//     //
//     goal_list: Vec<(String, i64)>,
//     //All values will be in seconds
//     plan_layout_expanded: Vec<(String, i64)>,
//     plan_layout_raw: Vec<(String, i64)>,

//     curr_action: Action,
//     //Pathing
//     path: Option<Vec<(i64, i64)>>,
// }
// impl Default for ShortTerm {
//     fn default() -> Self {
//         Self {
//             view_range: 0,
//             concept_forget: 0,
//             reflection_time: 0,
//             reflection_size: 0,
//             overlap: 0,
//             kw_event_reflect: 0,
//             kw_thought_reflect: 0,
//             recency: 0,
//             relevancy: 0,
//             importance: 0,
//             recency_decay: 0.0,
//             importance_trig_max: 0,
//             importance_trig_curr: 0,
//             importance_ele: 0,
//             thought_count: 0,
//             goal_list: Vec::new(),
//             plan_layout_expanded: Vec::new(),
//             plan_layout_raw: Vec::new(),
//             curr_action: Action::default(),
//             path: None,
//         }
//     }
// }
// impl ShortTerm {
//     pub fn new(
//         view_range: i64,
//         concept_forget: i64,
//         reflection_time: i64,
//         reflection_size: i64,
//         overlap: i64,
//         kw_event_reflect: i64,
//         kw_thought_reflect: i64,
//         recency: i64,
//         relevancy: i64,
//         importance: i64,
//         recency_decay: f32,
//         importance_trig_max: i64,
//         importance_trig_curr: i64,
//         importance_ele: i64,
//         thought_count: i64,
//         goal_list: Vec<(String, i64)>,
//         plan_layout_expanded: Vec<(String, i64)>,
//         plan_layout_raw: Vec<(String, i64)>,
//         curr_action: Action,
//         path: Option<Vec<(i64, i64)>>,
//     ) -> Self {
//         Self {
//             view_range,
//             concept_forget,
//             reflection_time,
//             reflection_size,
//             overlap,
//             kw_event_reflect,
//             kw_thought_reflect,
//             recency,
//             relevancy,
//             importance,
//             recency_decay,
//             importance_trig_max,
//             importance_trig_curr,
//             importance_ele,
//             thought_count,
//             goal_list,
//             plan_layout_expanded,
//             plan_layout_raw,
//             curr_action,
//             path,
//         }
//     }
//     pub fn get_curr_index(&self, advance: i64, raw: bool, curr_time: Option<Time>) -> i64 {
//         match &curr_time {
//             None => -1,
//             Some(time) => {
//                 let sec_elapsed = time.time() + advance;
//                 let mut seek: i64 = 0;
//                 let mut value = 0;
//                 let target = match raw {
//                     true => &self.plan_layout_raw,
//                     false => &self.plan_layout_expanded,
//                 };
//                 for (i, (_, duration)) in target.iter().enumerate() {
//                     seek += duration;
//                     if seek > sec_elapsed {
//                         value = i.try_into().unwrap()
//                     }
//                 }
//                 value
//             }
//         }
//     }
//     pub fn set_action(&mut self, action: Action) {
//         self.curr_action = action;
//         self.path = None;
//     }
//     pub fn get_action(&self) -> String {
//         self.curr_action.to_string()
//     }
// }
