use core::time;
use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};
pub struct Date {
    month: Month,
    day: i32,
}
pub struct Time {
    //Seconds Elapsed Since 0.0
    hour: i32,
    min: i32,
    sec: i32,
}
impl Date {
    pub fn add_days(&mut self, amount: i32) {
        let mut days_to_add = amount;
        while days_to_add > 0 {
            let days_in_current_month = self.month.get_days_in_month();
            if self.day + days_to_add > days_in_current_month {
                days_to_add -= days_in_current_month - self.day + 1;
                self.day = 1;
                self.month = self.month.next_month();
            } else {
                self.day += days_to_add;
                days_to_add = 0;
            }
        }
    }
}
impl Time {
    pub fn from_hms(datetime: (i32, i32, i32)) -> Time {
        Time {
            hour: datetime.0,
            min: datetime.1,
            sec: datetime.2,
        }
    }
    pub fn time(&self) -> i32 {
        self.hour * 60 * 60 + self.min * 60 + self.sec
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.min, self.sec)
    }
}
#[derive(PartialEq, PartialOrd, Debug)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}
impl Month {
    fn get_days_in_month(&self) -> i32 {
        match self {
            Month::January => 31,
            Month::February => 28,
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }
    fn next_month(&self) -> Month {
        match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }
}
// impl Add for Month{
//     type Output = Month;
//     fn add(self, rhs: Self) -> Self::Output {
//         let mut val = self;
//         for i in 0..(rhs as usize){
//             val = val.next_month();
//         }
//         val
//     }
// }
impl AddAssign for Month {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..(rhs as usize) {
            *self = self.next_month();
        }
    }
}
impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.min == other.min && self.sec == other.sec
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.min.partial_cmp(&other.min) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.sec.partial_cmp(&other.sec)
    }
}

impl Add for Time {
    type Output = (Time, i32);

    fn add(self, rhs: Self) -> Self::Output {
        let mut sec = self.sec + rhs.sec;
        let mut min = self.min + rhs.min + sec / 60;
        sec %= 60;

        let mut hour = self.hour + rhs.hour + min / 60;
        min %= 60;

        let mut day = hour / 24;
        (Time { hour, min, sec }, day)
    }
}

