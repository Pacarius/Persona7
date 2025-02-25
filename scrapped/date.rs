pub struct Date {
    month: Month,
    day: i64,
}
impl Date {
    pub fn add_days(&mut self, amount: i64) {
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
impl Add for Month {
    type Output = Month;
    fn add(self, rhs: Self) -> Self::Output {
        let mut val = self;
        for i in 0..(rhs as usize) {
            val = val.next_month();
        }
        val
    }
}
impl AddAssign for Month {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..(rhs as usize) {
            *self = self.next_month();
        }
    }
}
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.month == other.month && self.day == other.day
    }
}
impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.month.partial_cmp(&other.month) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.day.partial_cmp(&other.day)
    }
}
struct DateTime {
    time: Time,
    date: Date,
}
impl DateTime {
    pub fn addTime(&mut self, rhs: Time) {
        let mut day;
        (self.time, day) = self.time + rhs;
    }
    pub fn addDays(&mut self, days: i64) {
        self.date.add_days(days);
    }
}
impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.date.partial_cmp(&other.date) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.time.partial_cmp(&other.time)
    }
}
impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.date == other.date
    }
}
