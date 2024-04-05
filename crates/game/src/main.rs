// Copyright 2024 Natalie Baker // AGPLv3 //

use core::f32::consts::TAU;

use bevy::{
    color::palettes::css as Colors, pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*
};
use game::{projectile::prelude::*, damage::prelude::*, transform::prelude::*, interp_orientation, GameCameraBundle, Plane, PlayerBundle, PlayerController, PluginPlayer, PluginsGameCamera, Prism, ProjectionGame, ProjectionGameDebug};

pub const STYLE_BULLET: ProjectileStyle = ProjectileStyle::from_name("bullet");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PluginsGameCamera)
        .add_plugins(PluginPlayer)
        .add_plugins(PluginTransform)
        .add_plugins(PluginProjectile)
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, |
            mut commands: Commands, 
            time: Res<Time>, 
            mut accum: Local<f32>,
        | {
            *accum += time.delta_seconds();
            if *accum < 0.2 { return; }
            *accum -= 0.2;

            commands.spawn_projectile(Team::Enemy, STYLE_BULLET, 1.0, Vec2::new(10.0, 10.0), -25.0 * Vec2::Y);
        })
        .add_systems(PostUpdate, update_tilt.before(apply_transform_2ds))
        .run();
}

pub fn update_tilt(
    mut q: Query<(&mut Transform, &Transform2D)>, 
    time: Res<Time>,
) {
    q.iter_mut().for_each(|(mut t, c)| {
        let delta     = c.position.current - c.position.previous;
        t.translation = c.position.current.extend(0.0);
        t.rotation    = interp_orientation(t.rotation, calculate_ship_orientation_target(delta), 2.0*TAU*time.delta_seconds());
    });
}

pub fn setup(
    mut commands: Commands,
    mut projectile_styles: ResMut<ProjectileStyles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    projectile_styles.defs.insert(STYLE_BULLET, ProjectileStyleDefinition{
        shape: Shape::Circle(0.25),
        mesh: meshes.add(Sphere::new(0.25)),
        material_enemy:  materials.add(Color::from(Colors::RED)),
        material_player: materials.add(Color::from(Colors::AQUA)),
    });

    commands.spawn((
        GameCameraBundle{
            projection: ProjectionGame{
                planes_a: Plane::new( 0.0, Vec2::new(100.0, 100.0)),
                planes_b: Plane::new(50.0, Vec2::new(150.0, 150.0)),
                near: 0.1,
                far:  200.0,
                ..default()
            },
            ..default()
        },
        ProjectionGameDebug,
    ));

    commands.spawn((
        PlayerBundle {
            target: Target {
                shape: Shape::Circle(2.0),
                limit: f32::MAX, // DEBUG
                ..default()
            },
            controller: PlayerController {
                fire_style: STYLE_BULLET,
                ..default()
            },
            ..default()
        },
        PbrBundle {
            mesh: meshes.add(Prism{
                radius: 2.0,
                sides:  3,
                depth:  0.5,
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::from(Colors::RED)),
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
