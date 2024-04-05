// Copyright 2024 Natalie Baker // AGPLv3 //

use core::f32::consts::TAU;

use bevy::{
    color::palettes::css as Colors, pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*
};
use game::{prelude::*, GameCameraBundle, Plane, PlayerBundle, PlayerController, Prism, ProjectionGame, ProjectionGameDebug};

pub const STYLE_BULLET: ProjectileStyle = ProjectileStyle::from_name("bullet");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PluginsGame)
        .add_systems(Startup, setup)
        .add_systems(Update, (|
            mut commands: Commands, 
            time: Res<Time>, 
            mut accum: Local<f32>,
            player: Query<&Transform2D, With<PlayerController>>,
        | {
            *accum += time.delta_seconds();
            if *accum < 0.2 { return; }
            *accum -= 0.2;

            let player = player.single();
            let player_pos = player.position.previous;
            let player_vel = player.position.delta()/time.delta_seconds();
            let deg_45 = core::f32::consts::FRAC_PI_4;

            commands.spawn_projectile(
                Team::Enemy,        // Who owns this projectile?
                STYLE_BULLET,       // Projectile visuals / collider
                1.0,                // Projectile damage
                ProjectileAim::new(
                    Vec2::Y*45.0,   // Projectile firing origin
                    -Vec2::Y,       // Projectile direction
                    40.0            // Projectile speed
                ).aim_predictive(
                    player_pos,     // Target's starting position
                    player_vel,     // Target's estimated velocity
                    f32::MAX,       // Predict an unlimited distance into the future
                    deg_45,         // Limit aim to 45deg from direction
                )
            );
        }).after(SystemPlayerMovement))
        .add_systems(PostUpdate, update_tilt)
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
                limit: f32::MAX, // TODO for DEBUG testing
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
