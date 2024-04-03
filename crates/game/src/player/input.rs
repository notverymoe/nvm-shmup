// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct PlayerInput {
    pub move_dir: Vec2,
    pub fire:     bool,
}

pub fn prepare_player_input(mut q_player: Query<&mut PlayerInput>) {
    q_player.iter_mut().for_each(|mut player| player.move_dir = Vec2::ZERO);
}