// Copyright 2024 Natalie Baker // AGPLv3 //

pub mod styles;
pub mod plugin;
pub mod commands;

pub mod prelude {

    pub use super::styles::*;

    pub use super::plugin::*;

    pub use super::commands::{
        Team,
        CommandsSpawnProjectile
    };

}