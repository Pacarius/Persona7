use std::{fmt::Display, ops::Add};
#[derive(Clone, Copy, Debug)]
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
    pub fn time(&self) -> i64 {
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
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.min, self.sec)
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
// #[derive(Debug)]
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
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.month, self.day)
    }
}
// #[derive(Debug)]
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
