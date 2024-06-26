// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::{RayCaster, RayIntersection, ShapeCombined};

use crate::{damage::prelude::*, tags::prelude::*, transform::prelude::*};

use super::styles::ProjectileStyles;

pub struct PluginProjectile;

impl Plugin for PluginProjectile {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ProjectileStyles::default())
            .add_systems(Update, 
                (
                    integrate_projectiles_motion,
                    (
                        do_projectile_hits::<TeamEnemy >,
                        do_projectile_hits::<TeamPlayer>,
                    ),
                )
                .in_set(SystemProjectileUpdate)
                .chain()
            );
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct ProjectileDamage {
    pub shape:  Shape,
    pub amount: f32,
}

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct ProjectileSpeed(pub f32);

pub fn integrate_projectiles_motion(mut q: Query<(&mut Transform2D, &ProjectileSpeed)>, time: Res<Time>) {
    q.iter_mut().for_each(|(mut transform, &ProjectileSpeed(speed))| {
        let direction = transform.rotation.current;
        transform.position.current += direction * speed * time.delta_seconds();
    });
}

pub fn do_projectile_hits<Tag: Component>(
    mut q_sources: Query<(Entity, &ProjectileDamage, &Transform2D),    With<Tag>>,
    mut q_targets: Query<(Entity, &mut Target,       &Transform2D), Without<Tag>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // TODO OPT all of this. O(n^2)? Accel structure please?
    q_sources.iter_mut().for_each(|(entity_source, source, transform)| {
        let (dir, dist)  = transform.position.delta_as_bearing();
        let shape_source = source.shape.as_moving(transform.position.previous);
        let caster       = RayCaster::new(transform.position.previous, dir);
        let mut cache: Option<(Entity, RayIntersection)> = None;

        q_targets.iter().for_each(|(entity_target, target, transform)| {
            // OPT maybe we should make this a sparse_set flag?
            if !target.vulnerable(time.elapsed_seconds_f64()) {
                return;
            }

            // OPT LAZY we use a smeared shape for the sink. This is probably cheaper to calculate.
            //     but... we should do moving vs moving collisions for accurate TOI calculation for
            //     effects... but I'm not sure how to calculate the ray. Probably doesn't matter.
            let shape_target = source.shape.as_smear(transform.position.previous, transform.position.current);
            let combined     = ShapeCombined::between_moving_and_static(&shape_source, &shape_target);
            if let Some([enter, exit]) = caster.test(&combined) {
                // If exit is behind the origin of the ray, then the shape ie behind the ray. 
                if exit.distance >= 0.0 && enter.distance <= dist {
                    if let Some(prev) = cache {
                        if enter.distance < prev.1.distance { // Get first hit
                            cache = Some((entity_target, enter));
                        }
                    } else {
                        cache = Some((entity_target, enter));
                    }
                }
            }
        });

        if let Some(hit) = cache {
            let (_, mut target, _) = q_targets.get_mut(hit.0).unwrap();
            target.deal(time.elapsed_seconds_f64(), source.amount);
            commands.entity(entity_source).despawn();
        }

    });
}

