// Copyright 2024 Natalie Baker // AGPLv3 //

use core::ops::{Deref, DerefMut, Index, IndexMut};

use bevy::prelude::Vec2;

use super::{Curve, BezierOrder1, BezierOrder2, BezierOrder3};

#[derive(Debug, Clone, Copy)]
pub enum Bezier {
    Order1(BezierOrder1),
    Order2(BezierOrder2),
    Order3(BezierOrder3),
}

impl Bezier {

    #[must_use]
    pub const fn order_1(a: Vec2, b: Vec2) -> Self {
        Self::Order1(BezierOrder1::new(a, b))
    }

    #[must_use]
    pub const fn order_2(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self::Order2(BezierOrder2::new(a, b, c))
    }

    #[must_use]
    pub const fn order_3(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Self {
        Self::Order3(BezierOrder3::new(a, b, c, d))
    }

}

impl Deref for Bezier {
    type Target = [Vec2];
    fn deref(&self) -> &Self::Target {
        match self {
            Bezier::Order1(v) => &**v,
            Bezier::Order2(v) => &**v,
            Bezier::Order3(v) => &**v,
        }
    }
}

impl DerefMut for Bezier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Bezier::Order1(v) => &mut **v,
            Bezier::Order2(v) => &mut **v,
            Bezier::Order3(v) => &mut **v,
        }
    }
}

impl Index<usize> for Bezier {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Self::Output {
        &(**self)[index]
    }
}

impl IndexMut<usize> for Bezier {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (**self)[index]
    }
}

impl Curve for Bezier {
    fn interpolate(&self, t: f32) -> Vec2 {
        match self {
            Bezier::Order1(v) => v.interpolate(t),
            Bezier::Order2(v) => v.interpolate(t),
            Bezier::Order3(v) => v.interpolate(t),
        }
    }

    fn split(&self, t: f32) -> [Self; 2] {
        match self {
            Bezier::Order1(v) => v.split(t).map(Self::Order1),
            Bezier::Order2(v) => v.split(t).map(Self::Order2),
            Bezier::Order3(v) => v.split(t).map(Self::Order3),
        }
    }

    fn linearize(&self, result: &mut Vec<Vec2>, deviation_max: f32, prepend: bool) {
        match self {
            Bezier::Order1(v) => v.linearize(result, deviation_max, prepend),
            Bezier::Order2(v) => v.linearize(result, deviation_max, prepend),
            Bezier::Order3(v) => v.linearize(result, deviation_max, prepend),
        }
    }

    fn length_bounds(&self) -> [f32; 2] {
        match self {
            Bezier::Order1(v) => v.length_bounds(),
            Bezier::Order2(v) => v.length_bounds(),
            Bezier::Order3(v) => v.length_bounds(),
        }
    }
}
