// Copyright 2024 Natalie Baker // AGPLv3 //

use core::f32::consts::TAU;

use bevy::{
    color::palettes::css as Colours, pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*
};
use game::{apply_transform_2ds, calculate_ship_orientation_target, interp_orientation, GameCameraBundle, Plane, PlayerBundle, PluginPlayer, PluginProjectile, PluginTransform, PluginsGameCamera, Prism, ProjectionGame, Transform2D};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PluginsGameCamera)
        .add_plugins(PluginPlayer)
        .add_plugins(PluginTransform)
        .add_plugins(PluginProjectile)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, update_tilt.before(apply_transform_2ds))
        .run();
}

pub fn update_tilt(
    mut q: Query<(&mut Transform, &Transform2D)>, 
    time: Res<Time>,
) {
    q.iter_mut().for_each(|(mut t, c)| {
        let delta = c.position.current - c.position.target;
        t.translation = c.position.current.extend(0.0);
        t.rotation = interp_orientation(t.rotation, calculate_ship_orientation_target(delta), 2.0*TAU*time.delta_seconds());
    });
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    commands.spawn((
        PlayerBundle::default(),
        PbrBundle {
            mesh: meshes.add(Prism{
                radius: 0.75,
                sides:  3,
                depth:  0.5,
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::from(Colours::RED)),
            ..default()
        }
    ));

    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight{
            color: Color::WHITE,
            illuminance: AMBIENT_DAYLIGHT/2.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
