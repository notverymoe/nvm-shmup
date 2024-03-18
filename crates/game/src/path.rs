// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Clone)]
struct PathCacheEntry {
    from_start: f32,
    from_end:   f32,
    segment:    f32,
}

impl PathCacheEntry {
    #[must_use]
    pub const fn get_distance_to_segment(&self, forward: bool) -> f32 {
        if forward { 
            self.from_start 
        } else { 
            self.from_end 
        }
    }

    #[must_use]
    pub fn get_distance_to_segment_end(&self, forward: bool) -> f32 {
        self.segment + self.get_distance_to_segment(forward)
    }

    #[must_use]
    pub fn contains(&self, forward: bool, distance: f32) -> bool {
        let distance = distance - self.get_distance_to_segment(forward);
        (distance >= 0.0) && (distance < self.segment)
    }
}

#[must_use]
pub struct PositionQueryResult {
    pub index:     usize,
    pub position:  Vec2,
    pub distance:  f32,
    pub remaining: f32,
    pub loops:     u32,
    pub forward:   bool,
}

impl PositionQueryResult {

    pub const fn new(
        index:     usize,
        position:  Vec2,
        distance:  f32,
        remaining: f32,
        loops:     u32,
        forward:   bool,
    ) -> Self {
        Self{index, position, distance, remaining, loops, forward}
    }

}

#[derive(Debug, Clone)]
pub struct Path {
    path:  Box<[Vec2]>,
    cache: Box<[PathCacheEntry]>,
    distance_total: f32,
}

impl Path {
    pub fn new(v: impl Into<Box<[Vec2]>>) -> Self {
        let path: Box<[Vec2]> = v.into();
        assert!(path.len() > 1);
        let (cache, distance_total) = Self::calculate_cache(&path);
        Self{path, cache, distance_total}
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
    pub const fn segment(&self, idx: usize, forward: bool) -> Option<[Vec2; 2]> {
        if idx < self.len() {
            Some(self.segment_unchecked(idx, forward))
        } else {
            None
        }
    }

    #[must_use] 
    const fn segment_unchecked(&self, idx: usize, forward: bool) -> [Vec2; 2] {
        if forward {
            [self.path[idx], self.path[idx+1]]
        } else {
            [self.path[idx+1], self.path[idx]]
        }
    }

    #[must_use] 
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn find_position(&self, hint: Option<usize>, distance: f32, forward: bool, loop_max: Option<u32>, reverse_at_end: bool) -> PositionQueryResult {
        let loops    = self.resolve_loops(distance, loop_max);
        let distance = distance - self.distance_total*(loops as f32);
        let forward  = get_direction_after_loops(forward, reverse_at_end, loops);
        let hint     = if loops > 0 { None } else { hint };

        if let Some(i) = self.find_segment(hint, distance, forward) {
            let cache = &self.cache[i];
            let delta = (distance-cache.get_distance_to_segment(forward))/cache.segment;
            let [from, to] = self.segment_unchecked(i, forward);
            let position = from.lerp(to, delta);
            let remaining = distance - cache.get_distance_to_segment_end(forward);
            PositionQueryResult::new(i, position, distance, remaining, loops, forward)
        } else {
            let i = if forward { self.cache.len() - 1 } else { 0 };
            let cache    = &self.cache[i];
            let position = self.path[i];
            let remaining = distance - cache.get_distance_to_segment_end(forward);
            PositionQueryResult::new(i, position, distance, remaining, loops, forward)
        }
    }

    #[must_use]
    pub fn find_segment(&self, hint: Option<usize>, distance: f32, forward: bool) -> Option<usize> {
        if distance >= self.distance_total {
            return None;
        }

        let hint = self.resolve_hint(hint, forward);

        // If the "last" hint occurs after the distance in the direction we're moving, then we need to invert the search direction
        let search_dir = if distance < self.cache[hint].get_distance_to_segment(forward) { !forward } else { forward };

        if search_dir {
            self.cache.iter().skip(hint).position(|cache| cache.contains(forward, distance)).map(|i| hint + i)
        } else {
            let last = self.cache.len() - (hint + 1);
            self.cache.iter().rev().skip(last).position(|cache| cache.contains(forward, distance)).map(|i| self.cache.len() - (last + i + 1))
        }
    }

    #[must_use]
    pub const fn resolve_hint(&self, hint: Option<usize>, forward: bool) -> usize {
        if let Some(hint) = hint {
            hint
        } else if forward { 
            0 
        } else { 
            self.cache.len() - 1 
        }
    }

    #[must_use]
    pub fn resolve_loops(&self, distance: f32, loop_max: Option<u32>) -> u32 {
        ((distance / self.distance_total).floor() as u32).min(loop_max.unwrap_or(u32::MAX))
    }

    #[must_use]
    pub const fn points(&self) -> &[Vec2] {
        &self.path
    }

    pub fn segments(&'_ self, forward: bool) -> impl Iterator<Item = [Vec2; 2]> + '_ {
        let max = self.len() - 1;
        let range = if forward { 0..=max } else { max..=0 };
        range.map(move |i| self.segment(i, forward).unwrap())
    }
}

impl Path {
    #[must_use]
    fn calculate_cache(path: &[Vec2]) -> (Box<[PathCacheEntry]>, f32) {
        let mut dist_accum = 0.0;
        let mut cache = path.iter().enumerate().skip(1).map(|(i, v)| {
            let segment = v.distance(path[i-1]);
            let result = PathCacheEntry{
                from_start: dist_accum,
                from_end:   0.0,
                segment,
            };
            dist_accum += segment;
            result
        }).collect::<Vec<_>>().into_boxed_slice();

        for entry in cache.iter_mut() {
            entry.from_end = dist_accum - (entry.from_start + entry.segment);
        }

        (cache, dist_accum)
    }
}

#[must_use]
const fn get_direction_after_loops(forward: bool, reverse_at_end: bool, loops: u32) -> bool {
    if reverse_at_end { 
        if (loops % 2) == 0 { forward } else { !forward } 
    } else { 
        forward 
    }
}