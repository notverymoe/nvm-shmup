// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::Path;

#[derive(Debug, Clone, Component)]
pub struct PathFollower {
    path:     Path,
    last:     usize,
    distance: f32,
    speed:    f32,
    position: Vec2,
}

impl PathFollower {

    #[must_use]
    pub fn new(path: Path, speed: f32) -> Self {
        Self {
            position: path.start(),
            path,
            speed,
            last: 0,
            distance: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.last     = 0;
        self.distance = 0.0;
        self.position = self.path.start();
    }

    // Speed //

    #[must_use] 
    pub const fn speed(&self) -> f32 {
        self.speed
    }

    #[must_use] 
    pub fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    // Distance //

    #[must_use] 
    pub const fn distance(&self) -> f32 {
        self.distance
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
        let (position, last) = self.path.get_position(0, distance);
        self.last     = last;
        self.position = position;
    }

    // Last //

    #[must_use] 
    pub fn segment_current(&self) -> [Vec2; 2] {
        self.path.segment(self.last).unwrap_or_else(|| [self.path.end(), self.path.end()])
    }

    #[must_use] 
    pub const fn segment_current_idx(&self) -> usize {
        self.last
    }

    #[must_use] 
    pub const fn at_end(&self) -> bool {
        self.last > self.path.len()
    }


    // Path //

    #[must_use] 
    pub const fn path(&self) -> &Path {
        &self.path
    }
    
    // Position //

    #[must_use] 
    pub const fn position(&self) -> Vec2 {
        self.position
    }

}

pub fn path_follower_system(mut q_followers: Query<&mut PathFollower>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for mut follower in &mut q_followers {
        follower.distance += follower.speed*delta;
        let (position, last) = follower.path.get_position(follower.last, follower.distance);
        follower.position = position;
        follower.last     = last;
    }
}
