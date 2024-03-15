// Copyright 2024 Natalie Baker // AGPLv3 //

use core::ops::{Deref, DerefMut, Index, IndexMut};

use bevy::prelude::Vec2;

use super::{BezierOrder2, Curve};

#[derive(Debug, Clone, Copy)]
pub struct BezierOrder3([Vec2; 4]);

impl BezierOrder3 {

    #[must_use]
    pub const fn new(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Self {
        Self([a, b, c, d])
    }

}

impl Deref for BezierOrder3 {
    type Target = [Vec2; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BezierOrder3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for BezierOrder3 {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for BezierOrder3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Curve for BezierOrder3 {
    fn interpolate(&self, t: f32) -> Vec2 {
        (1.0-t)*BezierOrder2::new(self[0], self[1], self[2]).interpolate(t) 
            + t*BezierOrder2::new(self[1], self[2], self[3]).interpolate(t)
    }

    fn split(&self, t: f32) -> [Self; 2] {
        let [a, b, c, d] = **self;
        let e = t*(a+b);
        let f = t*(b+c);
        let g = t*(c+d);
        let h = t*(e+f);
        let j = t*(f+g);
        let k = t*(h+j);
        [
            Self([self[0], e, h, k]),
            Self([k, j, g, self[3]]),
        ]
    }

    fn linearize(&self, result: &mut Vec<Vec2>, deviation_max: f32, prepend: bool) {
        // OPT this is a guess, improve it
        let point_estimation = 2 + (self.deviation_from_linear()/deviation_max).ceil() as usize;
        result.reserve(point_estimation);
        if prepend { result.push(self[0]); }
        self.linearaize_internal(result, deviation_max);
    }

    fn length_bounds(&self) -> [f32; 2] {
        [
            self[0].distance(self[3]), 
            self[0].distance(self[1]) + self[1].distance(self[2]) + self[2].distance(self[3])
        ]
    }
}

impl BezierOrder3 {

    fn linearaize_internal(&self, result: &mut Vec<Vec2>, deviation_max: f32) {
        if self.deviation_from_linear() > deviation_max {
            let parts = self.split(0.5);
            parts[0].linearaize_internal(result, deviation_max);
            parts[1].linearaize_internal(result, deviation_max);
        } else {
            result.push(self[3]);
        }
    }

    fn deviation_from_linear(&self) -> f32 {
        let [min, max] = self.length_bounds();
        max - min
    }

}