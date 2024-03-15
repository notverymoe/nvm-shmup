// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_curve::{Bezier, Curve};

#[derive(Component)]
struct BezierData {
    curve:     Bezier,
    selected:  usize,
    deviation: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
            commands.spawn(BezierData{
                selected:  0,
                deviation: 1.0,
                curve: Bezier::order_3(
                    Vec2::new(-400.0,    0.0),
                    Vec2::new(-200.0,  200.0),
                    Vec2::new( 200.0, -200.0),
                    Vec2::new( 400.0,    0.0),
                )
            });
        })
        .add_systems(Update, |mut q_curves: Query<&mut BezierData>, mut gizmos: Gizmos, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>| {

            for mut data in &mut q_curves {

                // Point select
                if keys.just_pressed(KeyCode::Equal) {
                    data.selected = (data.selected + 1).min(data.curve.len() - 1);
                }
                if keys.just_pressed(KeyCode::Minus) {
                    data.selected = data.selected.saturating_sub(1);
                }

                // Deviation
                if keys.just_pressed(KeyCode::KeyQ) && data.deviation > 0.0625 {
                    data.deviation *= 0.5;
                }
                if keys.just_pressed(KeyCode::KeyE) && data.deviation < 1024.0  {
                    data.deviation *= 2.0;
                }

                // Mode 
                if keys.just_pressed(KeyCode::BracketLeft) {
                    data.curve = match data.curve {
                        Bezier::Order1(v) => Bezier::order_2(v[0], 0.5*(v[0]+v[1]), v[1]),
                        Bezier::Order2(v) => Bezier::order_3(v[0], v[1], v[1], v[2]),
                        Bezier::Order3(v) => Bezier::order_1(v[0], v[1]),
                    };
                    data.selected = data.selected.min(data.curve.len() - 1);
                }
                if keys.just_pressed(KeyCode::BracketRight) {
                    data.curve = match data.curve {
                        Bezier::Order1(v) => Bezier::order_3(v[0], (v[0]+v[1])/3.0, 2.0*(v[0]+v[1])/3.0, v[1]),
                        Bezier::Order2(v) => Bezier::order_1(v[0], v[2]),
                        Bezier::Order3(v) => Bezier::order_2(v[0], 0.5*(v[1]+v[2]), v[3]),
                    };
                    data.selected = data.selected.min(data.curve.len() - 1);
                }
                
                // Adjust
                let mut move_dir = Vec2::ZERO;
                if keys.pressed(KeyCode::KeyW) { move_dir += Vec2::Y; }
                if keys.pressed(KeyCode::KeyA) { move_dir -= Vec2::X; }
                if keys.pressed(KeyCode::KeyS) { move_dir -= Vec2::Y; }
                if keys.pressed(KeyCode::KeyD) { move_dir += Vec2::X; }
                if let Some(move_dir) = move_dir.try_normalize() {
                    let idx = data.selected.min(data.curve.len() - 1);
                    data.curve[idx] += 100.0 * move_dir * time.delta_seconds();
                }

                // Curve
                let mut points = Vec::default();
                data.curve.linearize(&mut points, data.deviation, true);
                gizmos.linestrip_2d(points.iter().copied(), Color::YELLOW);
                for point in points { gizmos.circle_2d(point, 4.0, Color::PINK); }

                // Points
                for (i, point) in data.curve.iter().copied().enumerate() {
                    gizmos.circle_2d(point, 8.0, if i == 0 || i == data.curve.len()-1 {
                        Color::RED
                    } else {
                        Color::ORANGE
                    });
                }
                gizmos.circle_2d(data.curve[data.selected], 12.0, Color::YELLOW);
            }
        })
        .run();
}
