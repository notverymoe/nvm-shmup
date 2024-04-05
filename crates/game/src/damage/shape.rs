// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle(f32),
    Box(Vec2),
}

impl Shape {

    #[must_use]
    pub fn as_smear(self, origin: Vec2, end: Vec2) -> ShapeStatic {
        let delta     = end - origin;
        let distance  = delta.length();

        if distance <= 0.0 {
            self.as_static(origin)
        } else {
            let direction = delta/distance;
            let size_smear = Vec2::new(0.0, distance/2.0);
            match self {
                Shape::Circle(size) => BoxOrientedRound::new(origin, size_smear,        direction, size).into(),
                Shape::Box(size)    =>      BoxOriented::new(origin, size_smear + size, direction      ).into(),
            }
        }

    }

    #[must_use]
    pub fn as_moving(self, origin: Vec2) -> ShapeMoving {
        match self {
            Self::Circle(size) => Ball::new(origin, size).into(),
            Self::Box(size)    => BoxAligned::new(origin, size).into(),
        }
    }

    #[must_use]
    pub fn as_static(self, origin: Vec2) -> ShapeStatic {
        match self {
            Self::Circle(size) => Ball::new(origin, size).into(),
            Self::Box(size)    => BoxAligned::new(origin, size).into(),
        }
    }

}

impl Default for Shape {
    fn default() -> Self {
        Self::Circle(0.5)
    }
}
