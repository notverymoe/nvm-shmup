// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::Cooldown;

use super::shape::Shape;

#[derive(Debug, Clone, Copy, Component)]
pub struct Target {
    pub shape:  Shape,
    pub damage: f32,
    pub limit:  f32,
    pub cooldown: Cooldown,
}

impl Default for Target {
    fn default() -> Self {
        Self { 
            shape: Shape::default(), 
            damage: 0.0, 
            limit: 1.0, 
            cooldown: Cooldown::default() 
        }
    }
}

impl Target {
    #[must_use]
    pub const fn new(shape: Shape, limit: f32, cooldown_duration: f64) -> Self {
        Self { 
            shape,
            damage: 0.0, 
            limit, 
            cooldown: Cooldown::new(cooldown_duration) 
        }
    }

    #[must_use]
    pub fn vulnerable(&self, time_seconds: f64) -> bool {
        !self.cooldown.active(time_seconds) && self.damage < self.limit
    }

    pub fn deal(&mut self, time_seconds: f64, amount: f32) {
        self.damage = (self.damage + amount).min(self.limit);
        self.cooldown.trigger(time_seconds);
    }
}


