use std::{
    error::Error,
    fmt::Display,
    ops::{Add, AddAssign, Sub},
};

use serde::Serialize;
#[derive(Clone, Copy, Debug, Eq, Serialize)]
pub struct Time {
    //Seconds Elapsed Since 0.0
    hour: i64,
    min: i64,
    sec: i64,
}
pub const DAY_LENGTH: i64 = 60 * 60 * 24;
impl Time {
    pub fn from_hms(datetime: (i64, i64, i64)) -> Time {
        Time {
            hour: datetime.0,
            min: datetime.1,
            sec: datetime.2,
        }
    }
    pub fn from_seconds(seconds: i64) -> Time {
        let hour = (seconds / 3600) % 24;
        let min = (seconds / 60) % 60;
        let sec = seconds % 60;
        Time { hour, min, sec }
    }
    pub fn in_seconds(&self) -> i64 {
        self.hour * 60 * 60 + self.min * 60 + self.sec
    }
    pub fn parse_time_pair(time_str: &str) -> Option<(Time, Time)> {
        let time_parts: Vec<_> = time_str.split(':').collect();
        if time_parts.len() == 3 {
            if let (Ok(hour), Ok(minute), Ok(second)) = (
                time_parts[0].parse::<i64>(),
                time_parts[1].parse::<i64>(),
                time_parts[2].parse::<i64>(),
            ) {
                let time = Time::from_hms((hour, minute, second));
                return Some((time, Time::from_seconds(DAY_LENGTH - 1)));
            }
        }
        None
    }
    pub fn diff(lhs: Time, rhs: Time) -> Time {
        let mut list = [lhs, rhs];
        list.sort_by(|a, b| a.in_seconds().cmp(&b.in_seconds()));
        list[1] - list[0]
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.hour, self.min, self.sec)
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
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hour.cmp(&other.hour) {
            std::cmp::Ordering::Equal => match self.min.cmp(&other.min) {
                std::cmp::Ordering::Equal => self.sec.cmp(&other.sec),
                other => other,
            },
            other => other,
        }
    }
}
impl Add for Time {
    type Output = (Time, i64);

    fn add(self, rhs: Self) -> Self::Output {
        let mut sec = self.sec + rhs.sec;
        let mut min = self.min + rhs.min + sec / 60;
        sec %= 60;

        let mut hour = self.hour + rhs.hour + min / 60;
        min %= 60;

        let day = hour / 24;
        hour %= 24;

        (Time { hour, min, sec }, day)
    }
}
impl AddAssign for Time {
    // type Output = Self;
    fn add_assign(&mut self, rhs: Self) {
        let (new_time, _) = *self + rhs;
        self.hour = new_time.hour;
        self.min = new_time.min;
        self.sec = new_time.sec;
    }
}
impl Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Self) -> Self::Output {
        let total_seconds_self = self.in_seconds();
        let total_seconds_rhs = rhs.in_seconds();
        let total_seconds_diff = total_seconds_self - total_seconds_rhs;
        Time::from_seconds(total_seconds_diff)
    }
}
// #[derive(Debug)]

#[derive(Clone, PartialEq, PartialOrd)]
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
    fn get_days_in_month(&self) -> i64 {
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
impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let month_str = match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        };
        write!(f, "{}", month_str)
    }
}
// #[derive(Debug)]
#[derive(Clone)]
pub struct Date {
    day: i64,
    month: Month,
}
impl Date {
    pub fn increment_month(&mut self) {
        self.month = self.month.next_month();
    }
    pub fn add_days(&mut self, mut target: i64) {
        while target > self.month.get_days_in_month() - self.day {
            target -= self.month.get_days_in_month() - self.day + 1;
            self.day = 1;
            self.increment_month();
        }
        self.day += target;
    }
    pub fn new(day: i64, month: Month) -> Self {
        Self { day, month }
    }
    pub fn cmp(&self, rhs: &Date) -> bool {
        //>>//
        self.month > rhs.month || self.day > rhs.day
    }
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.month, self.day)
    }
}
// #[derive(Debug)]
#[derive(Clone)]
pub struct DateTime(pub Date, pub Time);
impl DateTime {
    pub fn add(&mut self, time: Time) {
        let output = self.1 + time;
        self.1 = output.0;
        self.0.add_days(output.1);
    }
}
impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
impl Add<Time> for DateTime {
    type Output = (DateTime, bool);
    fn add(self, rhs: Time) -> Self::Output {
        // todo!()
        let (time, days) = self.1 + rhs;
        let mut new = DateTime(self.0, self.1);
        new.0.add_days(days);
        new.1 = time;
        (new, days > 0)
    }
}
pub fn weekday(date: &Date) -> String {
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let mut total_days = date.day;
    let mut current_month = Month::January;

    while current_month != date.month {
        total_days += current_month.get_days_in_month();
        current_month = current_month.next_month();
    }

    let day_of_week = total_days % 7;
    days[day_of_week as usize].to_string()
}
