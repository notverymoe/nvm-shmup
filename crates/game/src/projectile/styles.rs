// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, utils::HashMap};
use nvm_str_id::newtype_str_id;

use crate::Shape;

newtype_str_id!(pub ProjectileStyle);

#[derive(Debug, Clone, Resource)]
pub struct ProjectileStyles {
    pub defs: HashMap<ProjectileStyle, ProjectileStyleDefinition>,
}

#[derive(Debug, Clone)]
pub struct ProjectileStyleDefinition {
    pub shape:           Shape,
    pub mesh:            Handle<Mesh>,
    pub material_player: Handle<StandardMaterial>,
    pub material_enemy:  Handle<StandardMaterial>,
}

impl ProjectileStyleDefinition {

    #[must_use]
    pub const fn new(
        shape:           Shape,
        mesh:            Handle<Mesh>,
        material_player: Handle<StandardMaterial>,
        material_enemy:  Handle<StandardMaterial>,
    ) -> Self {
        Self { shape, mesh, material_player, material_enemy }
    }

}