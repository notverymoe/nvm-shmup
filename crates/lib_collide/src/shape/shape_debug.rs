// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

pub enum ShapeDebugData {
    Circle{
        origin: Vec2,
        radius: f32,
    },
    Polygon{
        points:  Box<[Vec2]>,
        normals: Box<[Vec2]>,
    },
    PolygonRound{
        points:  Box<[Vec2]>,
        normals: Box<[Vec2]>,
        radius:  f32,
    }
}

impl ShapeDebugData {

    pub fn circle(origin: Vec2, radius: f32) -> Self {
        Self::Circle{origin, radius}
    }

    pub fn polygon(points: Box<[Vec2]>, normals: Box<[Vec2]>) -> Self {
        Self::Polygon{points, normals}
    }

    pub fn polygon_round(points: Box<[Vec2]>, normals: Box<[Vec2]>, radius: f32) -> Self {
        Self::PolygonRound{points, normals, radius}
    }

    pub fn iter_segments(&self) -> impl Iterator<Item = ([Vec2; 3], f32)> + '_ {
        let ([points, normals], offset) = match self {
            ShapeDebugData::Circle { .. } => ([[].as_ref(), [].as_ref()], 0.0_f32),
            ShapeDebugData::Polygon { points, normals } => ([points.as_ref(), normals.as_ref()], 0.0_f32),
            ShapeDebugData::PolygonRound { points, normals, radius } => ([points.as_ref(), normals.as_ref()], *radius),
        };

        (0..points.len()).map(move |i| {
            let norm = normals[i];
            let from = points[i];
            let to   = points[(i+1) % points.len()];
            ([from, to, norm], offset)
        })
    }

}

pub trait ShapeDebug {
    fn get_debug_shape_data(&self) -> ShapeDebugData;
}