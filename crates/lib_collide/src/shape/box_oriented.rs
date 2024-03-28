// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, ShapeCommon, BoxAligned};

#[derive(Debug, Clone, Copy)]
pub struct BoxOriented {
    pub origin:    Vec2,
    pub size:      Vec2,
    pub direction: Vec2,
}

impl BoxOriented {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2) -> Self {
        Self{origin, size, direction}
    }
}

impl ShapeCommon for BoxOriented {
    fn bounding_box(&self) -> BoxAligned {
        let bound_x = Vec2::new( self.size.x,  self.size.y).rotate(self.direction).abs();
        let bound_y = Vec2::new(-self.size.x,  self.size.y).rotate(self.direction).abs();
        let size = bound_x.max(bound_y);
        BoxAligned::new(self.origin, size)
    }

    fn origin(&self) -> Vec2 {
        self.origin
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }
}

impl RayTarget for BoxOriented {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {

        // TODO OPT we might be able to rotate the ray into local space, then the intersections 
        //          out of, this would allow us to use the faster AABB check in adition to 
        //          having less rotate calls. This should also work for the rounded variant.

        let points = [
            self.origin + Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
            self.origin + Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
            self.origin + Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
            self.origin + Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
        ];

        let normals = [
            self.direction.perp(),
            -self.direction,
            -self.direction.perp(),
            self.direction
        ];

        let lengths = [
            2.0*self.size.x,
            2.0*self.size.y,
            2.0*self.size.x,
            2.0*self.size.y
        ];

        ray.test_polygon_at_origin(&points, &normals, &lengths)
    }
}

impl ShapeDebug for BoxOriented {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::polygon( 
            Box::new([
                self.origin + Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
                self.origin + Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
                self.origin + Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
                self.origin + Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
            ]), 
            Box::new([
                 self.direction.perp(),
                -self.direction,
                -self.direction.perp(),
                 self.direction
            ]),
        )
    }
}
