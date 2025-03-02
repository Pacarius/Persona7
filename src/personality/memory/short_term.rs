
use crate::misc::time::Time;
pub struct Action {
    location: String,
    start_time: Time,
    intended_duration: i64,
    description: String,
    description_emoji: String,
    //Object
    o_type: String,
    o_action: String,
    o_description_emoji: String,

    //Chat
    chat_target: Option<String>,
    chat_log: String,
    chat_target_buffer: Vec<String>,
    chat_end_time: i64,
}
impl Action {
    pub fn completed(&self, time: Time) -> bool {
        let (end_time, day) = self.start_time + Time::from_hms((0, 0, self.intended_duration));
        day > 0 || time >= end_time
    }
}
pub struct ShortTerm {
    view_range: i64,
    //Don't think this is relevant
    concept_forget: i64,
    reflection_time: i64,
    reflection_size: i64,
    overlap: i64,
    kw_event_reflect: i64,
    kw_thought_reflect: i64,
    //
    recency: i64,
    relevancy: i64,
    importance: i64,
    recency_decay: f32,
    importance_trig_max: i64,
    importance_trig_curr: i64,
    importance_ele: i64,
    thought_count: i64,
    //
    goal_list: Vec<(String, i64)>,
    //All values will be in seconds
    plan_layout_expanded: Vec<(String, i64)>,
    plan_layout_raw: Vec<(String, i64)>,

    curr_action: Action,
    //Pathing
    path: Option<Vec<(i64, i64)>>,
}
impl ShortTerm {
    pub fn get_curr_index(&self, advance: i64, raw: bool, curr_time: Option<Time>) -> i64 {
        match &curr_time {
            None => -1,
            Some(time) => {
                let sec_elapsed = time.time() + advance;
                let mut seek: i64 = 0;
                let mut value = 0;
                let target = match raw {
                    true => &self.plan_layout_raw,
                    false => &self.plan_layout_expanded,
                };
                for (i, (_, duration)) in target.iter().enumerate() {
                    seek += duration;
                    if seek > sec_elapsed {
                        value = i.try_into().unwrap()
                    }
                }
                value
            }
        }
    }
    pub fn set_action(&mut self, action: Action) {
        self.curr_action = action;
        self.path = None;
    }
}