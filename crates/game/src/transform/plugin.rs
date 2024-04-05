// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, transform::systems::{propagate_transforms, sync_simple_transforms}};

use super::prelude::*;

pub struct PluginTransform;

impl Plugin for PluginTransform {
    fn build(&self, app: &mut App) {
        app
            .add_systems(First,      apply_transform_2ds)
            .add_systems(PostUpdate, propogate_transform_2ds.before(propagate_transforms).before(sync_simple_transforms));
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct TransformSync;

pub fn propogate_transform_2ds(mut q: Query<(&mut Transform, &Transform2D), With<TransformSync>>) {
    q.iter_mut().for_each(|(mut t, t2d)| {
        t.translation = t2d.position.current.extend(0.0);
        t.rotation    = Quat::from_rotation_z(t2d.rotation.current.to_angle());
    });
}

pub fn apply_transform_2ds(mut q: Query<&mut Transform2D>) {
    q.iter_mut().for_each(|mut t| t.apply());
}
