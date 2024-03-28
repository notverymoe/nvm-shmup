// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;
use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::prelude::*;

macro_attr! {
    #[derive(EnumFromInner!, Debug, Clone, Copy)]
    pub enum ShapeMoving {
        Ball(Ball),
        BoxAligned(BoxAligned),
    }
}

impl ShapeMoving {

    pub fn origin(&self) -> Vec2 {
        match self {
            ShapeMoving::Ball(s)       => s.origin,
            ShapeMoving::BoxAligned(s) => s.origin,
        }
    }

}

impl ShapeCommon for ShapeMoving {
    fn bounding_box(&self) -> BoxAligned {
        match self {
            ShapeMoving::Ball(s)       => s.bounding_box(),
            ShapeMoving::BoxAligned(s) => s.bounding_box(),
        }
    }

    fn origin(&self) -> Vec2 {
        match self {
            ShapeMoving::Ball(s)       => s.origin(),
            ShapeMoving::BoxAligned(s) => s.origin(),
        }
    }

    fn set_origin(&mut self, origin: Vec2) {
        match self {
            ShapeMoving::Ball(s)       => s.set_origin(origin),
            ShapeMoving::BoxAligned(s) => s.set_origin(origin),
        }
    }
}

impl ShapeDebug for ShapeMoving {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        match self {
            ShapeMoving::Ball(s) => s.get_debug_shape_data(),
            ShapeMoving::BoxAligned(s) => s.get_debug_shape_data(),
        }
    }
}

impl RayTarget for ShapeMoving {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeMoving::Ball(s) => s.raycast(ray),
            ShapeMoving::BoxAligned(s) => s.raycast(ray),
        }
    }

    fn raycast_enter(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeMoving::Ball(s) => s.raycast_enter(ray),
            ShapeMoving::BoxAligned(s) => s.raycast_enter(ray),
        }
    }

    fn raycast_exit(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeMoving::Ball(s) => s.raycast_exit(ray),
            ShapeMoving::BoxAligned(s) => s.raycast_exit(ray),
        }
    }
}
