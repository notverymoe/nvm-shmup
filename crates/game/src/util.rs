// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[must_use]
pub fn calculate_ship_orientation_target(delta: Vec2) -> Quat {
    let tilt = delta.normalize_or_zero();
      Quat::from_axis_angle(Vec3::Y, -core::f32::consts::FRAC_PI_4*tilt.x)
    * Quat::from_axis_angle(Vec3::X,  core::f32::consts::FRAC_PI_4*tilt.y)
}