// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[must_use] 
pub fn axes_digital(dir: Vec2, deadzone: f32) -> Vec2 {
    if dir.length_squared() <= deadzone*deadzone {
        Vec2::ZERO
    } else {
        Vec2::from_angle((dir.to_angle()/core::f32::consts::FRAC_PI_4).round()*core::f32::consts::FRAC_PI_4)
    }
}

#[must_use] 
pub fn normalize_axis(amount: f32) -> f32 {
    amount.abs().min(1.0).max(0.0) * amount.signum()
}
