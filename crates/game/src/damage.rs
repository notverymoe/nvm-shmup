// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::{Ball, BoxAligned, BoxOriented, BoxOrientedRound, RayCaster, RayIntersection, ShapeCombined, ShapeMoving, ShapeStatic};

use crate::{update_player_firing, update_player_movement, Transform2D};

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle(f32),
    Box(Vec2),
}

impl Shape {

    #[must_use]
    pub fn as_smear(self, origin: Vec2, end: Vec2) -> ShapeStatic {
        let delta     = end - origin;
        let distance  = delta.length();

        if distance <= 0.0 {
            self.as_static(origin)
        } else {
            let direction = delta/distance;
            let size_smear = Vec2::new(0.0, distance/2.0);
            match self {
                Shape::Circle(size) => BoxOrientedRound::new(origin, size_smear,        direction, size).into(),
                Shape::Box(size)    =>      BoxOriented::new(origin, size_smear + size, direction      ).into(),
            }
        }

    }

    #[must_use]
    pub fn as_moving(self, origin: Vec2) -> ShapeMoving {
        match self {
            Self::Circle(size) => Ball::new(origin, size).into(),
            Self::Box(size)    => BoxAligned::new(origin, size).into(),
        }
    }

    #[must_use]
    pub fn as_static(self, origin: Vec2) -> ShapeStatic {
        match self {
            Self::Circle(size) => Ball::new(origin, size).into(),
            Self::Box(size)    => BoxAligned::new(origin, size).into(),
        }
    }

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

#[derive(Debug, Default, Clone, Component)]
pub struct DamageSourceOnce {
    pub shape:  Shape,
    pub amount: u32,
    pub hit:    Option<(Entity, Vec2)>,
}

#[derive(Debug, Default, Clone, Component)]
pub struct DamageTarget {
    pub shape: Shape,
    pub hits:  Vec<(Entity, u32)>,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct ProjectileLinear {
    pub velocity: Vec2,
}

#[derive(Debug, Clone, Bundle)]
pub struct BundleProjectile {
    pub transform:  Transform2D,
    pub projectile: ProjectileLinear,
    pub damage:     DamageSourceOnce,
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
                damage: DamageSourceOnce {
                    shape:  Shape::Circle(size),
                    amount: damage,
                    hit:    None,
                }
            }
        )
    }
}

pub struct PluginProjectile;

impl Plugin for PluginProjectile {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (
                update_projectiles_linear,
                (
                    (
                        update_damage_triggers::< TeamEnemy, TeamPlayer>,
                        update_damage_triggers::<TeamPlayer, TeamEnemy >,
                    ),
                    destroy_projectiles_after_hit,
                ).chain()
            )
            .chain()
            .after(update_player_movement)
            .after(update_player_firing)
        );
    }
}

pub fn update_projectiles_linear(mut q: Query<(&mut Transform2D, &ProjectileLinear)>, time: Res<Time>) {
    q.iter_mut().for_each(|(mut transform, projectile)| {
        transform.position.current += projectile.velocity * time.delta_seconds();
    });
}

pub fn destroy_projectiles_after_hit(q: Query<(Entity, &DamageSourceOnce), Changed<DamageSourceOnce>>, mut commands: Commands) {
    q.iter().for_each(|(entity, source)| {
        if source.hit.is_some() {
            // TODO spawn particles
            commands.entity(entity).despawn();
        }
    });
}

pub fn update_damage_triggers<Target: Component, Source: Component>(
    mut q_sources: Query<(Entity, &mut DamageSourceOnce, &Transform2D), With<Source>>,
    mut q_targets: Query<(Entity, &mut DamageTarget,     &Transform2D), With<Target>>,
) {

    
    // TODO OPT all of this
    q_sources.iter_mut().for_each(|(entity_source, mut source, transform)| {
        let (dir, dist)  = transform.position.delta_as_bearing();
        let shape_source = source.shape.as_moving(transform.position.previous);
        let caster       = RayCaster::new(transform.position.previous, dir);
        let mut cache: Option<(Entity, RayIntersection)> = None;

        q_targets.iter().for_each(|(entity_target, _, transform)| {
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
            target.hits.push((entity_source, source.amount));
            source.hit = Some((hit.0, hit.1.point));
        }

    });
}

