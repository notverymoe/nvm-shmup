// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{damage::prelude::*, tags::prelude::*, transform::prelude::*};

mod input_map;
pub use input_map::*;

mod controller;
pub use controller::*;

#[derive(Debug, Default, Bundle)]
pub struct PlayerBundle {
    pub input_config:  PlayerInputConfig,
    pub controller:    PlayerController,
    pub target:        Target, 
    pub transform:     Transform2D,
    pub team:          TeamPlayer,
}

pub struct PluginPlayer;

impl Plugin for PluginPlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                update_player_input.in_set(SystemPlayerMovement),
                update_player_movement.in_set(SystemPlayerMovement), 
                update_player_firing
            )
            .before(SystemProjectileUpdate)
            .chain()
        );
    }
}
