// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebugData, ShapeDebug, BoxAligned, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct BoxAlignedRound {
    pub origin: Vec2,
    pub size:   Vec2,
    pub radius: f32,
}

impl BoxAlignedRound {
    pub fn new(origin: Vec2, size: Vec2, radius: f32) -> Self {
        Self{origin, size, radius}
    }
}

impl ShapeCommon for BoxAlignedRound {
    fn bounding_box(&self) -> BoxAligned {
        BoxAligned::new(self.origin, Vec2::new(self.size.x + self.radius, self.size.y + self.radius))
    }

    fn origin(&self) -> Vec2 {
        self.origin
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }
}

impl RayTarget for BoxAlignedRound {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect_rounded(self.origin, self.size, self.radius)
    }
}

impl ShapeDebug for BoxAlignedRound {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::polygon_round( 
            Box::new([
                self.origin + Vec2::new( self.size.x,  self.size.y),
                self.origin + Vec2::new(-self.size.x,  self.size.y),
                self.origin + Vec2::new(-self.size.x, -self.size.y),
                self.origin + Vec2::new( self.size.x, -self.size.y),
            ]), 
            Box::new([
                 Vec2::Y,
                -Vec2::X,
                -Vec2::Y,
                 Vec2::X
            ]),
            self.radius,
        )
    }
}

