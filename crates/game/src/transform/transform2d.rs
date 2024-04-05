// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::prelude::*;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Transform2D {
    pub position: Position2D,
    pub rotation: Rotation2D,
}

impl Transform2D {

    #[must_use]
    pub const fn new(position: Vec2, rotation: Vec2) -> Self {
        Self {
            position: Position2D::new(position),
            rotation: Rotation2D::new(rotation),
        }
    }

    pub fn apply(&mut self) {
        self.position.apply();
        self.rotation.apply();
    }

}
