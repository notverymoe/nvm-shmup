// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::{Ball, BoxAligned, BoxOriented, BoxOrientedRound, RayCaster, ShapeCombined, ShapeMoving, ShapeStatic};

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

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct DamageSource {
    pub shape:  Shape,
    pub amount: u32,
    pub count:  u32,
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct DamageSink {
    pub shape:  Shape,
    pub amount: u32,
    pub count:  u32,
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
                    amount: damage,
                    count:  0
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

pub fn destroy_projectiles_after_hit(q: Query<(Entity, &DamageSource), Changed<DamageSource>>, mut commands: Commands) {
    q.iter().for_each(|(entity, source)| {
        if source.count > 0 {
            commands.entity(entity).despawn();
        }
    });
}

pub fn update_damage_triggers<Sink: Component, Source: Component>(
    mut q_sources: Query<(&mut DamageSource, &Transform2D), With<Source>>,
    mut q_sinks:   Query<(&mut DamageSink,   &Transform2D), With<Sink  >>,
) {
    // TODO OPT all of this
    q_sources.iter_mut().for_each(|(mut source, transform)| {
        let (dir, dist)  = transform.position.delta_as_bearing();
        let shape_source = source.shape.as_moving(transform.position.previous);
        let caster       = RayCaster::new(transform.position.current, dir);
        q_sinks.iter_mut().for_each(|(mut sink, transform)| {
            // OPT LAZY we use a smeared shape for the sink. This is probably cheaper to calculate.
            //     but... we should do moving vs moving collisions for accurate TOI calculation for
            //     effects... but I'm not sure how to calculate the ray.
            let shape_sink = source.shape.as_smear(transform.position.previous, transform.position.current);
            let combined = ShapeCombined::between_moving_and_static(&shape_source, &shape_sink);
            if let Some([enter, exit]) = caster.test(&combined) {
                // If exit is behind the origin of the ray, then the shape ie behind the ray. 
                if exit.distance >= 0.0 && enter.distance <= dist {
                    sink.amount  += source.amount;
                    sink.count   += 1;
                    source.count += 1;
                }
            }
        });
    });
}

