// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;
use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::prelude::*;

macro_attr! {
    #[derive(EnumFromInner!, Debug, Clone, Copy)]
    pub enum ShapeCombined {
        Ball(Ball),
        
        BoxAligned(BoxAligned),
        BoxAlignedRound(BoxAlignedRound),

        BoxOrientedRound(BoxOrientedRound),
        BoxOrientedBoxy(BoxOrientedBoxy),
        BoxOrientedBoxyRound(BoxOrientedBoxyRound),

        RampRound(RampRound),
        RampBoxy(RampBoxy),
        RampBoxyRound(RampBoxyRound),
    }
}

impl ShapeCommon for ShapeCombined {
    fn bounding_box(&self) -> BoxAligned {
        match self {
            ShapeCombined::Ball(s)       => s.bounding_box(),
            ShapeCombined::BoxAligned(s) => s.bounding_box(),
            ShapeCombined::BoxAlignedRound(s) => s.bounding_box(),
            ShapeCombined::BoxOrientedRound(s) => s.bounding_box(),
            ShapeCombined::BoxOrientedBoxy(s) => s.bounding_box(),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.bounding_box(),
            ShapeCombined::RampRound(s) => s.bounding_box(),
            ShapeCombined::RampBoxy(s) => s.bounding_box(),
            ShapeCombined::RampBoxyRound(s) => s.bounding_box(),
        }
    }

    fn origin(&self) -> Vec2 {
        match self {
            ShapeCombined::Ball(s)       => s.origin(),
            ShapeCombined::BoxAligned(s) => s.origin(),
            ShapeCombined::BoxAlignedRound(s) => s.origin(),
            ShapeCombined::BoxOrientedRound(s) => s.origin(),
            ShapeCombined::BoxOrientedBoxy(s) => s.origin(),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.origin(),
            ShapeCombined::RampRound(s) => s.origin(),
            ShapeCombined::RampBoxy(s) => s.origin(),
            ShapeCombined::RampBoxyRound(s) => s.origin(),
        }
    }

    fn set_origin(&mut self, origin: Vec2) {
        match self {
            ShapeCombined::Ball(s)       => s.set_origin(origin),
            ShapeCombined::BoxAligned(s) => s.set_origin(origin),
            ShapeCombined::BoxAlignedRound(s) => s.set_origin(origin),
            ShapeCombined::BoxOrientedRound(s) => s.set_origin(origin),
            ShapeCombined::BoxOrientedBoxy(s) => s.set_origin(origin),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.set_origin(origin),
            ShapeCombined::RampRound(s) => s.set_origin(origin),
            ShapeCombined::RampBoxy(s) => s.set_origin(origin),
            ShapeCombined::RampBoxyRound(s) => s.set_origin(origin),
        }
    }
}

impl ShapeDebug for ShapeCombined {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        match self {
            ShapeCombined::Ball(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxAligned(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxAlignedRound(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedRound(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedBoxy(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.get_debug_shape_data(),
            ShapeCombined::RampRound(s) => s.get_debug_shape_data(),
            ShapeCombined::RampBoxy(s) => s.get_debug_shape_data(),
            ShapeCombined::RampBoxyRound(s) => s.get_debug_shape_data(),
        }
    }
}

impl RayTarget for ShapeCombined {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeCombined::Ball(s) => s.raycast(ray),
            ShapeCombined::BoxAligned(s) => s.raycast(ray),
            ShapeCombined::BoxAlignedRound(s) => s.raycast(ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast(ray),
            ShapeCombined::BoxOrientedBoxy(s) => s.raycast(ray),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.raycast(ray),
            ShapeCombined::RampRound(s) => s.raycast(ray),
            ShapeCombined::RampBoxy(s) => s.raycast(ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast(ray),
        }
    }

    fn raycast_enter(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeCombined::Ball(s) => s.raycast_enter(ray),
            ShapeCombined::BoxAligned(s) => s.raycast_enter(ray),
            ShapeCombined::BoxAlignedRound(s) => s.raycast_enter(ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast_enter(ray),
            ShapeCombined::BoxOrientedBoxy(s) => s.raycast_enter(ray),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.raycast_enter(ray),
            ShapeCombined::RampRound(s) => s.raycast_enter(ray),
            ShapeCombined::RampBoxy(s) => s.raycast_enter(ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast_enter(ray),
        }
    }

    fn raycast_exit(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeCombined::Ball(s) => s.raycast_exit(ray),
            ShapeCombined::BoxAligned(s) => s.raycast_exit(ray),
            ShapeCombined::BoxAlignedRound(s) => s.raycast_exit(ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast_exit(ray),
            ShapeCombined::BoxOrientedBoxy(s) => s.raycast_exit(ray),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.raycast_exit(ray),
            ShapeCombined::RampRound(s) => s.raycast_exit(ray),
            ShapeCombined::RampBoxy(s) => s.raycast_exit(ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast_exit(ray),
        }
    }
}

impl ShapeCombined {

    pub fn between_moving_and_static(a: &ShapeMoving, b: &ShapeStatic) -> Self {
        match (a, b) {
            (ShapeMoving::Ball(a),       ShapeStatic::Ball(b)           ) => Ball::new(b.origin, a.radius+b.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::BoxAligned(b)     ) => BoxAlignedRound::new(b.origin, b.size, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::BoxAlignedRound(b)) => BoxAlignedRound::new(b.origin, b.size, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::Ball(b)           ) => BoxAlignedRound::new(b.origin, a.size, b.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxAligned(b)     ) => BoxAligned::new(b.origin, a.size + b.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxAlignedRound(b)) => BoxAlignedRound::new(b.origin, a.size + b.size, b.radius).into(),

            (ShapeMoving::Ball(a),       ShapeStatic::BoxOriented(b)     ) => BoxOrientedRound::new(b.origin, b.size, b.direction, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::BoxOrientedRound(b)) => BoxOrientedRound::new(b.origin, b.size, b.direction, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxOriented(b)     ) => BoxOrientedBoxy::new(b.origin, b.size, b.direction, a.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxOrientedRound(b)) => BoxOrientedBoxyRound::new(b.origin, b.size, b.direction, a.size, b.radius).into(),

            // TODO do we need to "invert" ramps?
            // NOTE seems not?
            (ShapeMoving::Ball(a),       ShapeStatic::Ramp(b)     ) => RampRound::new(b.origin, b.direction, b.length, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::RampRound(b)) => RampRound::new(b.origin, b.direction, b.length, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::Ramp(b)     ) => RampBoxy::new(b.origin + b.get_normal().signum()*a.size, b.direction, b.length, a.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::RampRound(b)) => RampBoxyRound::new(b.origin + b.get_normal().signum()*a.size, b.direction, b.length, a.size, b.radius).into(),
        } 
    }

    pub fn between_moving(a: &ShapeMoving, b: &ShapeMoving) -> Self {
        match (a, b) {
            (ShapeMoving::Ball(a),       ShapeMoving::Ball(b)      ) => Ball::new(b.origin, a.radius+b.radius).into(),
            (ShapeMoving::Ball(a),       ShapeMoving::BoxAligned(b)) => BoxAlignedRound::new(b.origin, b.size, a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeMoving::Ball(b)      ) => BoxAlignedRound::new(b.origin, a.size, b.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeMoving::BoxAligned(b)) => BoxAligned::new(b.origin, a.size + b.size).into(),
        }
    }

}