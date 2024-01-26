use std::fmt;

pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes) % 1440;
        let total_minutes = if total_minutes < 0 { total_minutes + 1440 } else { total_minutes };
        Clock { minutes: total_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

pub fn run() {
    let clock = Clock::new(10, 0);
    let updated_clock = clock.add_minutes(3);
    println!("{}", updated_clock);  // In ra: 10:03
}

impl Eq for Clock {}
