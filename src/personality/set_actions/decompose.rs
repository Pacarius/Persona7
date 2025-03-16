use crate::misc::{ollama::ollama::Ollama, time::DateTime};

impl crate::world::character::Character {
    pub async fn decompose_task(&mut self, llama: &Ollama, datetime: &DateTime) {
        //Called when new action starts.
        println!("{}", self.decompose(datetime));
    }
}
