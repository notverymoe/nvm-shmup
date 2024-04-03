// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{DamageTarget, TeamPlayer, Transform2D};

mod input_map;
pub use input_map::*;

mod input;
pub use input::*;

mod controller;
pub use controller::*;

#[derive(Debug, Default, Bundle)]
pub struct PlayerBundle {
    pub input_config:  PlayerInputConfig,
    pub input:         PlayerInput,
    pub controller:    PlayerController,
    pub damage_sink:   DamageTarget, 
    pub transform:     Transform2D,
    pub fire_cooldown: PlayerWeaponCooldown, 
    pub team:          TeamPlayer,
}

pub struct PluginPlayer;

impl Plugin for PluginPlayer {
    fn build(&self, app: &mut App) {
        app
            .add_systems(First,  prepare_player_input)
            .add_systems(Update, (update_player_input, update_player_movement, update_player_firing).chain());
    }
}
