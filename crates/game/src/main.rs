// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use game::{collider_renderer, GameCameraBundle, Plane, PlayerBundle, PluginsGameCamera, PluginPlayer, ProjectionGame};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PluginsGameCamera)
        .add_plugins(PluginPlayer)
        .add_systems(PostUpdate, collider_renderer)
        .add_systems(Startup, |mut commands: Commands|{
            commands.spawn((
                GameCameraBundle{
                    projection: ProjectionGame{
                        planes_a: Plane::new(  0.0, 160.0),
                        planes_b: Plane::new(100.0, 320.0),
                        near: 0.1,
                        far:  1000.0,
                        ..default()
                    },
                    ..default()
                },
            ));
            commands.spawn(PlayerBundle::default());
        })
        //.add_systems(PostUpdate, update_viewports)
        .run();
}

// #[derive(Component)]
// struct ViewportScaler;

// fn update_viewports(mut q_camera: Query<&mut Camera, With<ViewportScaler>>) {
//     q_camera.iter_mut().for_each(|mut camera| {
//         if let Some(size) = camera.physical_target_size() {
//             let scale = size.min_element();
//             let off = (size - UVec2::new(scale, scale))/2;
//             camera.viewport = Some(Viewport { physical_position: off, physical_size: UVec2::new(scale, scale), ..Default::default() });
//         }
//     });
// }