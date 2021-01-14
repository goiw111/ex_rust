use std::fmt;
use time::{ext::NumericalDuration, Time};

#[derive(Debug,PartialEq)]
pub struct Clock {
    time:   Time,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {time: Time::midnight() + hours.hours() + minutes.minutes()}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {time: self.time + minutes.minutes()}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.time.hour(), self.time.minute())
    }
}

