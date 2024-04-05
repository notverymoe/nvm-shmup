// Copyright 2024 Natalie Baker // AGPLv3 //

mod player;
pub use player::*;

mod camera;
pub use camera::*;

mod prism;
pub use prism::*;

mod cooldown;
pub use cooldown::*;

pub mod input;
pub mod transform;
pub mod damage;
pub mod projectile;
pub mod path;
pub mod tags;
pub mod plugin;

pub mod prelude {
    pub use super::input::prelude::*;
    pub use super::transform::prelude::*;
    pub use super::damage::prelude::*;
    pub use super::projectile::prelude::*;
    pub use super::tags::prelude::*;
    pub use super::plugin::*;
}

