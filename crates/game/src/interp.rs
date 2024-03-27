// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[must_use]
pub fn interp_orientation(source: Quat, target: Quat, rate: f32) -> Quat {
    let angle = source.angle_between(target);
    if rate <= 0.0 || rate >= angle {
        target
    } else {
        source.lerp(target, rate/angle)
    }
}

#[must_use]
pub fn interp_position(source: Vec3, target: Vec3, rate: f32) -> Vec3 {
    let dist = source.distance_squared(target);
    if rate <= 0.0 || rate*rate >= dist {
        target
    } else {
        source.lerp(target, rate/dist.sqrt())
    }
}