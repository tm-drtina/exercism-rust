#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn normalize(&mut self) {
        self.hours += self.minutes / 60;
        self.minutes %= 60;

        if self.minutes < 0 {
            self.minutes += 60;
            self.hours -= 1;
        }

        self.hours %= 24;
        if self.hours < 0 {
            self.hours += 24;
        }
    }

    fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }.normalized()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
        .normalized()
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
