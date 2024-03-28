// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;
use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::prelude::*;

macro_attr! {
    #[derive(EnumFromInner!, Debug, Copy, Clone)]
    pub enum ShapeStatic {
        Ball(Ball),
        BoxAligned(BoxAligned),
        BoxAlignedRound(BoxAlignedRound),
        BoxOriented(BoxOriented),
        BoxOrientedRound(BoxOrientedRound),
        Ramp(Ramp),
        RampRound(RampRound),
    }
}

impl ShapeCommon for ShapeStatic {
    fn bounding_box(&self) -> BoxAligned {
        match self {
            ShapeStatic::Ball(s)       => s.bounding_box(),
            ShapeStatic::BoxAligned(s) => s.bounding_box(),
            ShapeStatic::BoxAlignedRound(s) => s.bounding_box(),
            ShapeStatic::BoxOriented(s) => s.bounding_box(),
            ShapeStatic::BoxOrientedRound(s) => s.bounding_box(),
            ShapeStatic::Ramp(s) => s.bounding_box(),
            ShapeStatic::RampRound(s) => s.bounding_box(),
        }
    }

    fn origin(&self) -> Vec2 {
        match self {
            ShapeStatic::Ball(s)       => s.origin(),
            ShapeStatic::BoxAligned(s) => s.origin(),
            ShapeStatic::BoxAlignedRound(s) => s.origin(),
            ShapeStatic::BoxOriented(s) => s.origin(),
            ShapeStatic::BoxOrientedRound(s) => s.origin(),
            ShapeStatic::Ramp(s) => s.origin(),
            ShapeStatic::RampRound(s) => s.origin(),
        }
    }

    fn set_origin(&mut self, origin: Vec2) {
        match self {
            ShapeStatic::Ball(s)       => s.set_origin(origin),
            ShapeStatic::BoxAligned(s) => s.set_origin(origin),
            ShapeStatic::BoxAlignedRound(s) => s.set_origin(origin),
            ShapeStatic::BoxOriented(s) => s.set_origin(origin),
            ShapeStatic::BoxOrientedRound(s) => s.set_origin(origin),
            ShapeStatic::Ramp(s) => s.set_origin(origin),
            ShapeStatic::RampRound(s) => s.set_origin(origin),
        }
    }
}

impl ShapeDebug for ShapeStatic {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        match self {
            ShapeStatic::Ball(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxAligned(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxAlignedRound(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxOriented(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxOrientedRound(s) => s.get_debug_shape_data(),
            ShapeStatic::Ramp(s) => s.get_debug_shape_data(),
            ShapeStatic::RampRound(s) => s.get_debug_shape_data(),
        }
    }
}

impl RayTarget for ShapeStatic {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeStatic::Ball(s) => s.raycast(ray),
            ShapeStatic::BoxAligned(s) => s.raycast(ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast(ray),
            ShapeStatic::BoxOriented(s) => s.raycast(ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast(ray),
            ShapeStatic::Ramp(s) => s.raycast(ray),
            ShapeStatic::RampRound(s) => s.raycast(ray),
        }
    }

    fn raycast_enter(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeStatic::Ball(s) => s.raycast_enter(ray),
            ShapeStatic::BoxAligned(s) => s.raycast_enter(ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast_enter(ray),
            ShapeStatic::BoxOriented(s) => s.raycast_enter(ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast_enter(ray),
            ShapeStatic::Ramp(s) => s.raycast_enter(ray),
            ShapeStatic::RampRound(s) => s.raycast_enter(ray),
        }
    }

    fn raycast_exit(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeStatic::Ball(s) => s.raycast_exit(ray),
            ShapeStatic::BoxAligned(s) => s.raycast_exit(ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast_exit(ray),
            ShapeStatic::BoxOriented(s) => s.raycast_exit(ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast_exit(ray),
            ShapeStatic::Ramp(s) => s.raycast_exit(ray),
            ShapeStatic::RampRound(s) => s.raycast_exit(ray),
        }
    }
}
