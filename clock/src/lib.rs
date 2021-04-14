use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut real_hours = hours;
        let mut real_mins = minutes;

        while real_mins < 0 {
            real_mins = real_mins + 60;
            real_hours = real_hours - 1;
        }

        while real_hours < 0 {
            real_hours = real_hours + 24;
        }

        real_hours = (real_hours + (real_mins / 60)) % 24;
        real_mins = real_mins % 60;

        Clock {
            hours: real_hours,
            minutes: real_mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, minutes + self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DIGIT_PADDING: usize = 2;
        write!(
            f,
            "{:0width$}:{:0width$}",
            self.hours,
            self.minutes,
            width = DIGIT_PADDING
        )
    }
}
