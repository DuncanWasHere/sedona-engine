use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: f32,
}

impl Time {
    pub const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    pub fn new(year: u32, month: u8, day: u8, hour: u8, minute: u8, second: f32) -> Self {
        let mut time = Time {
            year,
            month,
            day,
            hour,
            minute,
            second,
        };
        time.validate();
        time
    }

    fn validate(&mut self) {
        if self.second >= 60.0 {
            self.increment_seconds((self.second / 1.0).floor() as u32);
            self.second %= 60.0;
        }
        self.minute = self.minute.min(59);
        self.hour = self.hour.min(23);
        self.month = self.month.clamp(1, 12);
        let max_day = Self::DAYS_IN_MONTH[(self.month - 1) as usize];
        self.day = self.day.clamp(1, max_day);
    }

    pub fn increment_seconds(&mut self, secs: u32) {
        let total_seconds = self.second + secs as f32;
        let int_seconds = total_seconds.floor() as u32;
        self.second = total_seconds % 60.0;
        self.increment_minutes(int_seconds / 60);
    }

    pub fn increment_seconds_f32(&mut self, secs: f32) {
        let total_seconds = self.second + secs;
        let int_part = total_seconds.floor() as u32;
        self.second = total_seconds % 60.0;
        self.increment_minutes(int_part / 60);
    }

    pub fn increment_minutes(&mut self, mins: u32) {
        let total_minutes = self.minute as u32 + mins;
        self.minute = (total_minutes % 60) as u8;
        self.increment_hours(total_minutes / 60);
    }

    pub fn increment_hours(&mut self, hours: u32) {
        let total_hours = self.hour as u32 + hours;
        self.hour = (total_hours % 24) as u8;
        self.increment_days(total_hours / 24);
    }

    pub fn increment_days(&mut self, mut days: u32) {
        while days > 0 {
            let dim = Self::DAYS_IN_MONTH[(self.month - 1) as usize] as u32;
            if self.day as u32 + days <= dim {
                self.day += days as u8;
                return;
            } else {
                days -= dim - self.day as u32 + 1;
                self.day = 1;
                self.increment_months(1);
            }
        }
    }

    pub fn increment_months(&mut self, months: u32) {
        let total_month = self.month as u32 + months;
        self.year += (total_month - 1) / 12;
        self.month = ((total_month - 1) % 12 + 1) as u8;

        let max_day = Self::DAYS_IN_MONTH[(self.month - 1) as usize];
        if self.day > max_day {
            self.day = max_day;
        }
    }

    pub fn increment_years(&mut self, years: u32) {
        self.year += years;
    }

    pub fn day_of_year(&self) -> u32 {
        Self::DAYS_IN_MONTH[..(self.month - 1) as usize]
            .iter()
            .map(|&d| d as u32)
            .sum::<u32>()
            + self.day as u32
    }

    pub fn day_of_week(&self) -> u8 {
        // Zeller's Congruence for Gregorian calendar, adjusted to 0=Sunday
        let y = if self.month < 3 {
            self.year - 1
        } else {
            self.year
        };
        let m = if self.month < 3 {
            self.month + 12
        } else {
            self.month
        };
        let k = y % 100;
        let j = y / 100;
        let h = (self.day as u32 + ((13 * (m as u32 + 1)) / 5) + k + (k / 4) + (j / 4) + 5 * j) % 7;
        ((h + 6) % 7) as u8 // 0 = Sunday
    }

    pub fn time_of_day_fraction(&self) -> f32 {
        let total_seconds = (self.hour as f32 * 3600.0) + (self.minute as f32 * 60.0) + self.second;
        total_seconds / 86400.0
    }

    pub fn year(&self) -> &u32 {
        &self.year
    }

    pub fn month(&self) -> &u8 {
        &self.month
    }

    pub fn day(&self) -> &u8 {
        &self.day
    }

    pub fn hour(&self) -> &u8 {
        &self.hour
    }

    pub fn minute(&self) -> &u8 {
        &self.minute
    }

    pub fn second(&self) -> &f32 {
        &self.second
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:05.2}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
