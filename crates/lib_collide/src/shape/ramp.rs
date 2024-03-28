// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, get_polygon_data_for_ramp, ShapeCommon, BoxAligned};

#[derive(Debug, Clone, Copy)]
pub struct Ramp {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
}

impl Ramp {
    pub fn new(origin: Vec2, direction: Vec2, length: f32) -> Self {
        Self{origin, direction, length}
    }

    pub fn new_from_size(origin: Vec2, direction: Vec2, size: Vec2) -> Self {
        let length = size.length();
        let direction = Vec2::new(direction.x, -direction.y)*(size/length);
        Self{origin, direction, length}
    }

    pub fn new_from_size_centered(origin: Vec2, direction: Vec2, size: Vec2) -> Self {
        Self::new_from_size(origin - direction*size*0.5, direction, size)
    }

    pub fn get_normal(&self) -> Vec2 {
        let size = Vec2::new(self.direction.x, -self.direction.y) * self.length;
        if (size.x >= 0.0) == (size.y >= 0.0) {
            self.direction.perp()
        } else {
            -self.direction.perp()
        }
    }
}

impl ShapeCommon for Ramp {
    fn bounding_box(&self) -> BoxAligned {
        BoxAligned::new(self.origin, self.direction*self.length*0.5)
    }

    fn origin(&self) -> Vec2 {
        self.origin
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }
}

impl RayTarget for Ramp {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ray.test_polygon(self.origin, &points, &normals, &lengths)
    }
}

impl ShapeDebug for Ramp {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        let (points, normals, _lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ShapeDebugData::polygon(
            Box::new(points.map(|v| self.origin + v)), 
            Box::new(normals),
        )
    }
}