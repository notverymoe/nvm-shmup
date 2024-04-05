// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[must_use]
pub fn calculate_ship_orientation_target(delta: Vec2) -> Quat {
    let tilt = delta.normalize_or_zero();
      Quat::from_axis_angle(Vec3::Y,  core::f32::consts::FRAC_PI_4*tilt.x)
    * Quat::from_axis_angle(Vec3::X, -core::f32::consts::FRAC_PI_4*tilt.y)
}

#[must_use]
pub fn angle_between(source: f32, target: f32) -> f32 {
    let angle = (target - source).rem_euclid(core::f32::consts::TAU);
    if angle.abs() > core::f32::consts::PI { 
        angle - angle.signum()*core::f32::consts::TAU 
    } else { 
        angle 
    }
}

#[must_use]
pub fn interp_angles(source: f32, target: f32, rate: f32) -> f32 {
    let angle = angle_between(source, target);
    if rate <= 0.0 || rate >= angle.abs() {
        target
    } else {
        source + rate*angle.signum()
    }
}

#[must_use]
pub fn clamp_angles(source: f32, direction: f32, range: f32) -> f32 {
    let angle = angle_between(direction, source);
    direction + angle.signum()*angle.abs().min(range)
}

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