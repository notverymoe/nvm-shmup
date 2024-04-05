// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use game::path::Path;
use nvm_curve::{Bezier, Curve};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(PreUpdate, path_follower_system)
        .add_systems(Update, |q_followers: Query<&PathFollower>, mut gizmos: Gizmos| {
            for follower in &q_followers {
                gizmos.circle_2d(follower.position(), 2.0, Color::linear_rgb(1.0, 1.0, 0.0));
                for (i, [from, to]) in follower.path().segments(true).enumerate() {
                    gizmos.line_2d(from, to, if i == follower.segment_current_idx() { Color::linear_rgb(0.0, 1.0, 0.0) } else { Color::linear_rgb(0.0, 0.25, 0.0) });
                }
            }
        })
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
            commands.spawn(PathFollower::new(create_path(), 512.0, false, LoopBehaviour::ForeverReverse));
        })
        .run();
}

fn create_path() -> Path {
    let mut path = Vec::<_>::default();
    for (i, curve) in [
        Bezier::order_3(Vec2::ZERO,      Vec2::new(-135.0, 100.0), Vec2::new( 135.0, 200.0), Vec2::Y * 300.0),
        Bezier::order_3(Vec2::Y * 300.0, Vec2::new(-200.0, 200.0), Vec2::new(-200.0,   0.0), Vec2::ZERO     ),
    ].iter().enumerate() {
        curve.linearize(&mut path, 0.05, i == 0);
    }
    Path::new(path)
}

#[derive(Debug, Clone, Component)]
pub struct PathFollower {
    path:     Path,
    last:     Option<usize>,
    distance: f32,
    speed:    f32,
    position: Vec2,
    forward:  bool,
    loop_behaviour: LoopBehaviour,
    loop_count: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum LoopBehaviour {
    None,
    Forever,
    ForeverReverse,
    Count(u32),
    CountReverse(u32),
}

impl LoopBehaviour {

    #[must_use]
    pub const fn get_max_loops(&self, loop_count: u32) -> Option<u32> {
        match self {
            LoopBehaviour::None                => Some(0),
            LoopBehaviour::Forever             => None,
            LoopBehaviour::Count(count)        => Some(count.saturating_sub(loop_count)),
            LoopBehaviour::ForeverReverse      => None,
            LoopBehaviour::CountReverse(count) => Some(count.saturating_sub(loop_count)),
        }
    }

    #[must_use]
    pub const fn get_should_reverse(&self) -> bool {
        match self {
            LoopBehaviour::None            => false,
            LoopBehaviour::Forever         => false,
            LoopBehaviour::Count(_)        => false,
            LoopBehaviour::ForeverReverse  => true,
            LoopBehaviour::CountReverse(_) => true,
        }
    }

}

impl PathFollower {

    #[must_use]
    pub fn new(path: Path, speed: f32, forward: bool, loop_behaviour: LoopBehaviour) -> Self {
        Self {
            position: path.start(),
            path,
            speed,
            forward,
            loop_behaviour,
            last: None,
            loop_count: 0,
            distance: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.last       = None;
        self.distance   = 0.0;
        self.position   = self.path.start();
        self.loop_count = 0;
    }

    // Forward //
    #[must_use] 
    pub const fn forward(&self) -> bool {
        self.forward
    }
 
    pub fn set_forward(&mut self, forward: bool) {
        // TODO recalculate state
        self.forward = forward;
    }

    // Loop Behaviour //

    #[must_use] 
    pub const fn loop_behaviour(&self) -> LoopBehaviour {
        self.loop_behaviour
    }

    #[must_use] 
    pub fn loop_behaviour_mut(&mut self) -> &mut LoopBehaviour {
        &mut self.loop_behaviour
    }
 
    pub fn set_loop_behaviour(&mut self, loop_behaviour: LoopBehaviour) {
        self.loop_behaviour = loop_behaviour;
    }

    // Loop Count //

    #[must_use] 
    pub const fn loop_count(&self) -> u32 {
        self.loop_count
    }

    #[must_use] 
    pub fn loop_count_mut(&mut self) -> &mut u32 {
        &mut self.loop_count
    }
 
    pub fn set_loop_count(&mut self, count: u32) {
        self.loop_count = count;
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
        let result    = self.path.find_position(None, distance, self.forward, self.loop_behaviour.get_max_loops(0), self.loop_behaviour.get_should_reverse());
        self.last     = Some(result.index);
        self.position = result.position;
        self.distance = result.distance;
    }

    // Last //

    #[must_use] 
    pub fn segment_current(&self) -> [Vec2; 2] {
        self.path.segment(self.path.resolve_hint(self.last, self.forward), self.forward).unwrap()
    }

    #[must_use] 
    pub const fn segment_current_idx(&self) -> usize {
        self.path.resolve_hint(self.last, self.forward)
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

    // Helpers //

    pub fn advance(&mut self, delta_t: f32) {
        self.distance += self.speed * delta_t;
        let result = self.path.find_position(
            self.last, 
            self.distance, 
            self.forward,
            self.loop_behaviour.get_max_loops(self.loop_count),
            self.loop_behaviour.get_should_reverse(),
        );
        self.position    = result.position;
        self.last        = Some(result.index);
        self.distance    = result.distance;
        self.loop_count += result.loops;
        self.forward     = result.forward;
    }

}

pub fn path_follower_system(mut q_followers: Query<&mut PathFollower>, time: Res<Time>) {
    let delta_t = time.delta_seconds();
    for mut follower in &mut q_followers {
        if follower.speed > 0.0 {
            follower.advance(delta_t);
        }
    }
}
