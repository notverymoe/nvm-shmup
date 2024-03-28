// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, color::palettes::css as Colors};

use nvm_collide::prelude::*;

pub trait ShapeMarkerTrait: ShapeDebug + RayTarget + Send + Sync {}
impl<T: ShapeDebug + RayTarget + Send + Sync + 'static> ShapeMarkerTrait for T {}

pub fn render_shape(gizmos: &mut Gizmos, shape: &dyn ShapeMarkerTrait, colour: impl Into<Color>) {
    let colour = colour.into();
    let data = shape.get_debug_shape_data();
    match data {
        ShapeDebugData::Circle { origin, radius } => { 
            gizmos.circle_2d(origin, radius, colour); 
        },
        ShapeDebugData::Polygon { .. } => {
            let ShapeDebugData::Polygon { points, .. } = &data else { unreachable!() };
            gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), colour);
            for ([from, to, norm], offset) in data.iter_segments() {
                let off = to - from;
                let off_n = off.normalize();
                let near = offset + from + off_n*10.0;
                let far  = offset + to   - off_n*10.0;

                let mid = offset + from + off*0.5;
                gizmos.line_2d(mid, mid + norm*20.0, Colors::BLUE);
                gizmos.circle_2d(near, 5.0, Colors::ALICE_BLUE);
                gizmos.circle_2d(far, 5.0, Colors::TEAL);
            }
        },
        ShapeDebugData::PolygonRound { radius, .. } => {
            for ([from, to, norm], offset) in data.iter_segments() {
                let offset = norm * offset;
                if radius > 0.0 {
                    gizmos.circle_2d(from, radius, Colors::GREEN);
                }
                gizmos.line_2d(offset + from, offset + to, colour);
                let mid = offset + (from + to)*0.5;
                gizmos.line_2d(mid, mid + norm*20.0, Colors::BLUE);
            }
        },
    };
}

#[allow(dead_code)]
fn main() {}