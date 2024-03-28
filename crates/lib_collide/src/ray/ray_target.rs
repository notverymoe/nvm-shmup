// Copyright 2023 Natalie Baker // AGPLv3 //

use super::{RayCaster, RayIntersection};

pub trait RayTarget {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]>;

    fn raycast_enter(&self, ray: &RayCaster) -> Option<RayIntersection> {
        self.raycast(ray).map(|[v, _]| v)
    }

    fn raycast_exit(&self, ray: &RayCaster) -> Option<RayIntersection>{
        self.raycast(ray).map(|[_, v]| v)
    }
}