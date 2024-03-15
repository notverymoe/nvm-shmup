// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use game::{path_follower_system, Path, PathFollower};
use nvm_curve::{Bezier, Curve};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreUpdate, path_follower_system)
        .add_systems(Update, |q_followers: Query<&PathFollower>, mut gizmos: Gizmos| {
            for follower in &q_followers {
                gizmos.circle_2d(follower.position(), 2.0, Color::YELLOW);
                for (i, [from, to]) in follower.path().segments().enumerate() {
                    gizmos.line_2d(from, to, if i == follower.segment_current_idx() { Color::GREEN } else { Color::DARK_GREEN });
                }
            }
        })
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
            commands.spawn(PathFollower::new(create_path(), 32.0,));
        })
        .run();
}

fn create_path() -> Path {
    let mut path = Vec::<_>::default();
    for (i, curve) in [
        Bezier::order_3(Vec2::ZERO,      Vec2::new( 135.0,   0.0), Vec2::new( 135.0, 200.0), Vec2::Y * 200.0),
        Bezier::order_3(Vec2::Y * 200.0, Vec2::new(-135.0, 200.0), Vec2::new(-135.0,   0.0), Vec2::ZERO     ),
    ].iter().enumerate() {
        curve.linearize(&mut path, 0.05, i == 0);
    }
    Path::new(path)
}
