// Copyright 2024 Natalie Baker // AGPLv3 //

use core::ops::{Deref, DerefMut, Index, IndexMut};

use bevy::prelude::Vec2;

use super::Curve;

#[derive(Debug, Clone, Copy)]
pub struct BezierOrder1([Vec2; 2]);

impl BezierOrder1 {

    #[must_use]
    pub const fn new(a: Vec2, b: Vec2) -> Self {
        Self([a, b])
    }

}

impl Deref for BezierOrder1 {
    type Target = [Vec2; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BezierOrder1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for BezierOrder1 {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for BezierOrder1 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Curve for BezierOrder1 {
    fn interpolate(&self, t: f32) -> Vec2 {
        self[0] + t*(self[1] - self[0])
    }

    fn split(&self, t: f32) -> [Self; 2] {
        let mid = self.interpolate(t);
        [Self([self[0], mid]), Self([mid, self[1]])]
    }

    fn linearize(&self, result: &mut Vec<Vec2>, _deviation_max: f32, prepend: bool) {
        if prepend { 
            result.extend_from_slice(&**self);
        } else {
            result.push(self[1]);
        }
    }

    fn length_bounds(&self) -> [f32; 2] {
        let len = self[0].distance(self[1]);
        [len, len]
    }
}