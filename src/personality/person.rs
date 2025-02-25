use crate::misc::time::Time;

use super::mem::ShortTerm;

pub struct Person{
    location : (i64, i64),
    first_name: String,
    last_name: String,
    age : i64,
    core_traits : Vec<String>,
    stable_traits : Vec<String>,
    current_action : String,
    lifestyle : String,
    living_area : String,
    short_term_mem : ShortTerm,
    time : Option<Time>,
    daily_plan_req : Option<i64>,    
}

impl Person{
    pub fn get_iss(&self) -> String{
        format!("
        Name: {} {}
        Age: {}
        Innate Traits: {:?}
        Learned Traits: {:?}
        Currently: {}
        Lifestyle: {}
        Daily Plan Requirement: {:?}
        Current Date: {}
        ", self.first_name, 
        self.last_name, 
        self.age, 
        self.core_traits, 
        self.stable_traits, 
        self.current_action,
        self.lifestyle,
        self.daily_plan_req,
        match &self.time{
            None => "None".to_string(),
            Some(time) => time.to_string().clone()
        }
    )
    }
}