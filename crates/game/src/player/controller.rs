// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, color::palettes::css as Colors};

use crate::{BundleProjectile, TeamPlayer, Transform2D, TransformSync};

use super::input::PlayerInput;

#[derive(Debug, Component)]
pub struct PlayerController {
    pub move_speed: f32,
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PlayerWeaponCooldown {
    pub accum: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self { 
            move_speed: 20.0,
        }
    }
}

pub fn update_player_movement(
    mut q_player: Query<(&mut Transform2D, &PlayerController, &PlayerInput)>,
    time: Res<Time>,
) {
    q_player.iter_mut().for_each(|(mut transform, controller, input)| {
        if let Some(move_dir) = input.move_dir.try_normalize() {
            transform.position.current += move_dir * controller.move_speed * time.delta_seconds();
        }
    });
}

pub fn update_player_firing(
    mut q_player: Query<(&mut PlayerWeaponCooldown, &Transform2D, &PlayerInput)>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    q_player.iter_mut().for_each(|(mut cooldown, transform, input)| {
        if cooldown.accum > 0.0 {
            cooldown.accum = (cooldown.accum - time.delta_seconds()).max(0.0);
            if cooldown.accum > 0.0 {
                return;
            }
        }

        if input.fire {
            cooldown.accum = 0.1;
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