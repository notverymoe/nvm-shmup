// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Position2D {
    pub current: Vec2,
    pub target:  Vec2,
}

impl Position2D {
    #[must_use]
    pub const fn new(position: Vec2) -> Self {
        Self { current: position, target: position }
    }

    pub fn move_rel(&mut self, delta: Vec2) {
        self.target = self.current + delta;
    }

    pub fn apply(&mut self) {
        self.current = self.target;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rotation2D {
    pub current: Vec2,
    pub target:  Vec2,
}

impl Default for Rotation2D {
    fn default() -> Self {
        Self { current: Vec2::Y, target: Vec2::Y }
    }
}

impl Rotation2D {
    #[must_use]
    pub const fn new(rotation: Vec2) -> Self {
        Self { current: rotation, target: rotation }
    }

    pub fn apply(&mut self) {
        self.current = self.target;
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Transform2D {
    pub position: Position2D,
    pub rotation: Rotation2D,
}

impl Transform2D {

    #[must_use]
    pub const fn new(position: Vec2, rotation: Vec2) -> Self {
        Self {
            position: Position2D::new(position),
            rotation: Rotation2D::new(rotation),
        }
    }

    pub fn apply(&mut self) {
        self.position.apply();
        self.rotation.apply();
    }

}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct TransformSync;

pub fn sync_transforms(mut q: Query<(&mut Transform, &Transform2D), With<TransformSync>>) {
    q.iter_mut().for_each(|(mut t, t2d)| {
        t.translation = t2d.position.current.extend(0.0);
        t.rotation    = Quat::from_rotation_z(t2d.rotation.current.to_angle());
    });
}

pub fn apply_transform_2ds(mut q: Query<&mut Transform2D>) {
    q.iter_mut().for_each(|mut t| t.apply());
}

pub struct PluginTransform;

impl Plugin for PluginTransform {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (apply_transform_2ds, sync_transforms).chain());
    }
}