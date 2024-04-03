// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{color::palettes::css as Colors, prelude::*};

use crate::{BundleProjectile, Cooldown, TeamPlayer, Transform2D, TransformSync};

#[derive(Debug, Component)]
pub struct PlayerController {
    pub move_speed:    f32,
    pub move_dir:      Vec2,
    pub fire:          bool,
    pub fire_cooldown: Cooldown,
}


impl Default for PlayerController {
    fn default() -> Self {
        Self { 
            move_speed:    20.0,
            move_dir:      Vec2::ZERO,
            fire:          false,
            fire_cooldown: Cooldown::new(0.2),
        }
    }
}

pub fn update_player_movement(
    mut q_player: Query<(&mut Transform2D, &mut PlayerController)>,
    time: Res<Time>,
) {
    q_player.iter_mut().for_each(|(mut transform, mut controller)| {
        if let Some(move_dir) = core::mem::take(&mut controller.move_dir).try_normalize() {
            transform.position.current += move_dir * controller.move_speed * time.delta_seconds();
        }
    });
}

pub fn update_player_firing(
    mut q_player: Query<(&mut PlayerController, &Transform2D)>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    q_player.iter_mut().for_each(|(mut controller, transform)| {
        let fire = core::mem::take(&mut controller.fire);
        if controller.fire_cooldown.active() && !controller.fire_cooldown.reduce(time.delta_seconds()) {
            return;
        }

        if fire {
            controller.fire_cooldown.trigger();
            commands.spawn((
                BundleProjectile::bullet(TeamPlayer, transform.position.current, Vec2::Y * 100.0, 0.25, 1),
                PbrBundle { // TODO improve on this
                    mesh: meshes.add(Sphere::new(0.25)),
                    transform: Transform::from_translation(transform.position.current.extend(0.0)),
                    material: materials.add(Color::from(Colors::BLUE)),
                    ..default()
                },
                TransformSync
            ));
        }
    });
}