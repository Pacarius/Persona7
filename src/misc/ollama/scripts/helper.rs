use std::fmt::Display;

use crate::world::character::Character;
impl Character {
    pub fn rest(&self) -> String {
        let source = format!(
            "{common}
    Daily Plan: Today, {name} is planning to do the following five things: {daily:#?}
    Return {name}'s sleep time in HH:MM:SS form.",
            common = self.get_descriptor(),
            name = self.name,
            daily = self.short_term_mem().get_daily()
        );
        source
    }
}
