// Copyright 2024 Natalie Baker // AGPLv3 //

#[derive(Debug, Default, Copy, Clone)]
pub struct Cooldown {
    pub duration: f64,
    pub timeout:  f64,
}

impl Cooldown {

    #[must_use]
    pub const fn new(duration: f64) -> Self {
        Self{duration, timeout: 0.0}
    }

    #[must_use]
    pub fn active(&self, time_seconds: f64) -> bool {
        self.timeout > time_seconds
    }

    pub fn trigger(&mut self, time_seconds: f64) {
        self.timeout = time_seconds + self.duration;
    }
}