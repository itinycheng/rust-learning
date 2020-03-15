use std::fmt;
use std::fmt::{Display, Formatter};

const HOUR: i32 = 60;
const DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: u32,
}

impl Clock {
    fn std_minutes(minutes: i32) -> i32 {
        match minutes % DAY {
            min if min < 0 => min + DAY,
            min => min,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: Self::std_minutes(hours * HOUR + minutes) as u32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Self::std_minutes(self.minutes as i32 + minutes) as u32,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / HOUR as u32,
            self.minutes % HOUR as u32
        )
    }
}
