// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Path {
    path:  Box<[Vec2]>,
    cache: Box<[[f32; 2]]>,
}

impl Path {
    pub fn new(v: impl Into<Box<[Vec2]>>) -> Self {
        let path: Box<[Vec2]> = v.into();
        assert!(path.len() > 1);
        let cache = Self::calculate_cache(&path);
        Self{path, cache}
    }

    #[must_use] 
    pub const fn start(&self) -> Vec2 {
        self.path[0]
    }

    #[must_use] 
    pub const fn end(&self) -> Vec2 {
        self.path[self.path.len()-1]
    }

    #[must_use] 
    pub fn segment(&self, idx: usize) -> Option<[Vec2; 2]> {
        Some([
            *self.path.get(idx)?,
            *self.path.get(idx+1)?
        ])
    }

    #[must_use] 
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.cache.len()
    }

    #[must_use]
    pub fn get_position(&self, last: usize, distance: f32) -> (Vec2, usize) {
        if let Some(i) = self.find_segment(last, distance) {
            let [seg_dist, total_dist] = self.cache[i];
            let from  = self.path[i  ];
            let to    = self.path[i+1];
            let delta = (distance-total_dist)/seg_dist;
            (from.lerp(to, delta), i)
        } else {
            (self.path[self.path.len()-1], self.cache.len())
        }
    }

    #[must_use]
    pub fn find_segment(&self, last: usize, distance: f32) -> Option<usize> {
        self.cache.iter().skip(last).position(|[dist, total]| distance < total+dist).map(|i| last + i)
    }

    #[must_use]
    pub const fn points(&self) -> &[Vec2] {
        &self.path
    }

    pub fn segments(&'_ self) -> impl Iterator<Item = [Vec2; 2]> + '_ {
        (0..self.len()).map(|i| self.segment(i).unwrap())
    }
}

impl Path {
    fn calculate_cache(path: &[Vec2]) -> Box<[[f32; 2]]> {
        let mut dist_accum = 0.0;
        path.iter().enumerate().skip(1).map(|(i, v)| {
            let dist = v.distance(path[i-1]);
            let result = [dist, dist_accum];
            dist_accum += dist;
            result
        }).collect::<Vec<_>>().into_boxed_slice()
    }
}
