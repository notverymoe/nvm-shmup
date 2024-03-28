// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, get_polygon_data_for_ramp, ShapeCommon, BoxAligned};

#[derive(Debug, Clone, Copy)]
pub struct RampRound {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
    pub radius:    f32,
}

impl RampRound {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, radius: f32) -> Self {
        Self{origin, direction, length, radius}
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

impl ShapeCommon for RampRound {
    fn bounding_box(&self) -> BoxAligned {
        let size = self.direction*self.length*0.5;
        BoxAligned::new(self.origin, Vec2::new(size.x + self.radius, size.y + self.radius))
    }

    fn origin(&self) -> Vec2 {
        self.origin
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }
}

impl RayTarget for RampRound {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ray.test_polygon_rounded(self.origin, &points, &normals, &lengths, self.radius)
    }
}

impl ShapeDebug for RampRound {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        let (points, normals, _lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ShapeDebugData::polygon_round( 
            Box::new(points.map(|v| self.origin + v)), 
            Box::new(normals),
            self.radius,
        )
    }
}
