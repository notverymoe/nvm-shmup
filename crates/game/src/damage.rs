// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::Transform2D;

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle(f32),
    Box(Vec2),
}

impl Default for Shape {
    fn default() -> Self {
        Self::Circle(0.5)
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct TeamPlayer;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct TeamEnemy;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct DamageSource {
    pub shape:  Shape,
    pub amount: u32,
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct DamageSink {
    pub shape:  Shape,
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct ProjectileLinear {
    pub velocity: Vec2,
}

#[derive(Debug, Clone, Copy, Bundle)]
pub struct BundleProjectile {
    pub transform:  Transform2D,
    pub projectile: ProjectileLinear,
    pub damage:     DamageSource,
}

impl BundleProjectile {
    #[must_use]
    pub fn bullet<T: Component>(
        team:     T,
        origin:   Vec2,
        velocity: Vec2,
        size:     f32,
        damage:   u32,
    ) -> (T, Self) {
        (
            team,
            Self {
                transform:  Transform2D::new(origin, Vec2::X),
                projectile: ProjectileLinear{ velocity },
                damage: DamageSource {
                    shape:  Shape::Circle(size/2.0),
                    amount: damage
                }
            }
        )
    }
}

pub struct PluginProjectile;

impl Plugin for PluginProjectile {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_projectiles_linear);
    }
}

pub fn update_projectiles_linear(mut q: Query<(&mut Transform2D, &ProjectileLinear)>, time: Res<Time>) {
    q.iter_mut().for_each(|(mut transform, projectile)| {
        transform.position.move_rel(projectile.velocity * time.delta_seconds());
    });
}
