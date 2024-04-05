// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, utils::HashMap};

use super::styles::ProjectileStyles;

pub struct PluginProjectilesNew;

impl Plugin for PluginProjectilesNew {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProjectileStyles{
            defs: HashMap::default(),
        });
    }
}
