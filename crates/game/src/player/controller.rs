// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{projectile::prelude::*, transform::prelude::*, Cooldown};

#[derive(Debug, Component)]
pub struct PlayerController {
    pub move_speed:    f32,
    pub move_dir:      Vec2,
    pub fire:          bool,
    pub fire_cooldown: Cooldown,
    pub fire_style:    ProjectileStyle,
}


impl Default for PlayerController {
    fn default() -> Self {
        Self { 
            move_speed:    20.0,
            move_dir:      Vec2::ZERO,
            fire:          false,
            fire_cooldown: Cooldown::new(0.2),
            fire_style:    "".into(),
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
) {
    q_player.iter_mut().for_each(|(mut controller, transform)| {
        let fire = core::mem::take(&mut controller.fire);
        if controller.fire_cooldown.active(time.elapsed_seconds_f64()) {
            return;
        }

        if fire {
            controller.fire_cooldown.trigger(time.elapsed_seconds_f64());
            commands.spawn_projectile(Team::Player, controller.fire_style, 1.0, ProjectileAim::new(transform.position.current, Vec2::Y, 100.0));
        }
    });
}