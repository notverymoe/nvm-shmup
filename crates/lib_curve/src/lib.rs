// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

mod bezier;
pub use bezier::*;

mod bezier_order_1;
pub use bezier_order_1::*;

mod bezier_order_2;
pub use bezier_order_2::*;

mod bezier_order_3;
pub use bezier_order_3::*;

pub trait Curve: Sized {
    fn interpolate(&self, t: f32) -> Vec2;
    fn split(&self, t: f32) -> [Self; 2];
    fn linearize(&self, result: &mut Vec<Vec2>, deviation_max: f32, prepend: bool);
    fn length_bounds(&self) -> [f32; 2];
}
