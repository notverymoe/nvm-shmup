// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{BoxAligned, RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub origin: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn new(origin: Vec2, radius: f32) -> Self {
        Self{origin, radius}
    } 
}

impl ShapeCommon for Ball {
    fn bounding_box(&self) -> BoxAligned {
        BoxAligned::new(self.origin, Vec2::new(self.radius, self.radius))
    }

    fn origin(&self) -> Vec2 {
        self.origin
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }
}

impl RayTarget for Ball {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_circle(self.origin, self.radius)
    }
}

impl ShapeDebug for Ball {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::circle(self.origin, self.radius)
    }
}