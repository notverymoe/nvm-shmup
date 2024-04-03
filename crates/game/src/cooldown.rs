// Copyright 2024 Natalie Baker // AGPLv3 //

#[derive(Debug, Copy, Clone)]
pub struct Cooldown {
    pub length:  f32,
    pub current: f32,
}

impl Cooldown {

    #[must_use]
    pub const fn new(length: f32) -> Self {
        Self{length, current: 0.0}
    }

    #[must_use]
    pub fn is_active(&self) -> bool {
        self.current > 0.0
    }

    pub fn update(&mut self, delta_s: f32) -> bool {
        self.current = (self.current - delta_s).max(0.0);
        self.current <= 0.0
    }

    pub fn trigger(&mut self) {
        self.current = self.length;
    }

}