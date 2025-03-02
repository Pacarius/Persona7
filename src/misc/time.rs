use std::{fmt::Display, ops::Add};
#[derive(Clone, Copy)]
pub struct Time {
    //Seconds Elapsed Since 0.0
    hour: i64,
    min: i64,
    sec: i64,
}
impl Time {
    pub fn from_hms(datetime: (i64, i64, i64)) -> Time {
        Time {
            hour: datetime.0,
            min: datetime.1,
            sec: datetime.2,
        }
    }
    pub fn time(&self) -> i64 {
        self.hour * 60 * 60 + self.min * 60 + self.sec
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

        let hour = self.hour + rhs.hour + min / 60;
        min %= 60;

        let day = hour / 24;
        (Time { hour, min, sec }, day)
    }
}
