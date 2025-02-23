use crate::misc::time::Time;
pub struct Action{
    location : String,
    start_time : Time,
    intended_duration : i64,
    description : String,
    description_emoji : String,
    //Object
    o_type : String,
    o_action : String,
    o_description_emoji : String,

    //Chat
    chat_target : Option<String>,
    chat_log : String,
    chat_target_buffer : Vec<String>,
    chat_end_time : i64,
}
impl Action{
    pub fn completed(&self, time: Time){
        // let end_time = match self.chat_end_time
    }
}
pub struct ShortTerm{
    view_range : i32,
    //Don't think this is relevant
    concept_forget : i8,
    reflection_time : i8,
    reflection_size : i8,
    overlap : i8,
    kw_event_reflect : i8,
    kw_thought_reflect : i8,
    //
    recency : i8,
    relevancy : i8,
    importance : i8,
    recency_decay : f32,
    importance_trig_max : i8,
    importance_trig_curr : i8,
    importance_ele : i8,
    thought_count : i8,
    //
    goal_list : Vec<(String, i32)>,
    //All values will be in seconds
    plan_layout_expanded : Vec<(String, i32)>,
    plan_layout_raw : Vec<(String, i32)>,

    curr_action : Action,
    //Pathing
    path : Option<Vec<(i32, i32)>>,
}
impl ShortTerm{
    pub fn get_curr_index(&self, advance: i32, raw: bool, curr_time: Option<Time>) -> i32{
        match &curr_time{
            None => -1,
            Some(time) => {
                let sec_elapsed = time.time() + advance;
                let mut seek: i32 = 0;
                let mut value = 0;
                let target = match raw{
                    true => &self.plan_layout_raw,
                    false => &self.plan_layout_expanded
                };
                for (i, (_, duration)) in target.iter().enumerate(){
                    seek += duration;
                    if seek > sec_elapsed{
                        value = i.try_into().unwrap()
                    }
                }
                value
            }
        }
    }
    pub fn set_action(&mut self, action: Action){
        self.curr_action = action;
        self.path = None;
    }
}