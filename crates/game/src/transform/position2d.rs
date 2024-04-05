// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Position2D {
    pub current:  Vec2,
    pub previous: Vec2,
}

impl Position2D {
    #[must_use]
    pub const fn new(position: Vec2) -> Self {
        Self { current: position, previous: position }
    }

    pub fn apply(&mut self) {
        self.previous = self.current;
    }

    #[must_use]
    pub fn delta(&self) -> Vec2 {
        self.current - self.previous
    }

    #[must_use]
    pub fn delta_as_bearing(&self) -> (Vec2, f32) {
        let delta     = self.current - self.previous;
        let distance  = delta.length();
        let direction = if distance > 0.0 { delta/distance } else { Vec2::ZERO };
        (direction, distance)
    }
}