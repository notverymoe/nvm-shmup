// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebugData, ShapeDebug, get_polygon_data_for_ramp_boxy, PolygonSmallRound, PolygonSmall, BoxAligned, ShapeCommon};

pub struct RampBoxyRound(PolygonSmallRound);

impl RampBoxyRound {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, size: Vec2, radius: f32) -> Self {
        let (points, normals, lengths) = get_polygon_data_for_ramp_boxy(direction, length, size);

        let (min, max) = points.iter().fold((Vec2::MAX, Vec2::MIN), |p, &c| (p.0.min(c), p.0.max(c)));
        let bound_origin = (min + max)*0.5;
        let size         = max - bound_origin + Vec2::new(radius, radius);

        Self(PolygonSmallRound::new(PolygonSmall::new(points.map(|v| origin + v), normals, lengths, BoxAligned::new(bound_origin, size)), radius))
    }
}

impl ShapeCommon for RampBoxyRound {
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

impl RayTarget for RampBoxyRound {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(ray)
    }
}

impl ShapeDebug for RampBoxyRound {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        self.0.get_debug_shape_data()
    }
}
