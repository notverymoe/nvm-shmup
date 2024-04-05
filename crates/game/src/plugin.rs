// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::{prelude::*, PluginPlayer, PluginsGameCamera};

pub struct PluginsGame;

impl PluginGroup for PluginsGame {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(PluginsGameCamera)
            .add(PluginPlayer)
            .add(PluginTransform)
            .add(PluginProjectile)
            .build()
    }
}