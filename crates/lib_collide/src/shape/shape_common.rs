// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::BoxAligned;

pub trait ShapeCommon {
    fn bounding_box(&self) -> BoxAligned;
    fn origin(&self) -> Vec2;
    fn set_origin(&mut self, origin: Vec2);
}