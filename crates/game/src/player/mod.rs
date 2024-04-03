// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{DamageTarget, TeamPlayer, Transform2D};

mod input_button;
pub use input_button::*;

mod input_axis;
pub use input_axis::*;

mod input_map;
pub use input_map::*;

mod input;
pub use input::*;

mod controller;
pub use controller::*;

mod util;
pub use util::*;

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
            .add_systems(Update, (update_keyboard_input, update_player_movement, update_player_firing).chain());
    }
}
