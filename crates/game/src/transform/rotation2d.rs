// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Rotation2D {
    pub current: Vec2,
    pub target:  Vec2,
}

impl Default for Rotation2D {
    fn default() -> Self {
        Self { current: Vec2::Y, target: Vec2::Y }
    }
}

impl Rotation2D {
    #[must_use]
    pub const fn new(rotation: Vec2) -> Self {
        Self { current: rotation, target: rotation }
    }

    pub fn apply(&mut self) {
        self.current = self.target;
    }
}
