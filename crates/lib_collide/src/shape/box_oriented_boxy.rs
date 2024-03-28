// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, get_polygon_data_for_oriented_rect_rected, PolygonSmall, ShapeCommon, BoxAligned};

pub struct BoxOrientedBoxy(PolygonSmall);

impl BoxOrientedBoxy {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2, outer_size: Vec2) -> Self {
        Self(PolygonSmall::new_from_points(get_polygon_data_for_oriented_rect_rected(origin, size, direction, outer_size)))
    }
}

impl ShapeCommon for BoxOrientedBoxy {
    fn bounding_box(&self) -> BoxAligned {
        self.0.bounding_box()
    }

    fn origin(&self) -> Vec2 {
        self.0.origin()
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.0.set_origin(origin);
    }
}

impl RayTarget for BoxOrientedBoxy {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(ray)
    }
}

impl ShapeDebug for BoxOrientedBoxy {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        self.0.get_debug_shape_data()
    }
}
