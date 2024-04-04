// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{ecs::world::Command, prelude::*, utils::hashbrown::HashMap};

use crate::{DamageSourceOnce, ProjectileLinear, TeamEnemy, TeamPlayer, Transform2D, TransformSync};

mod styles;
pub use styles::*;

pub struct PluginProjectilesNew;

impl Plugin for PluginProjectilesNew {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProjectileStyles{
            defs: HashMap::default(),
        });
    }
}

// #[derive(Debug, Clone, Copy, Component)]
// pub struct ProjectileCollider {
//     pub shape:  Shape,
//     pub amount: u32,
// }

// #[derive(Debug, Clone, Copy, Component)]
// pub struct ProjectileVelocity {
//     pub velocity: Vec2,
// }

#[derive(Debug, Clone, Copy)]
pub enum Team {
    Player,
    Enemy
}

pub struct SpawnProjectile<T: Bundle = ()> {
    pub team:     Team,
    pub style:    ProjectileStyle,
    pub damage:   u32,
    pub origin:   Vec2,
    pub velocity: Vec2,
    pub additional: T,
}

impl SpawnProjectile<()> {

    #[must_use]
    pub const fn new(
        team: Team,
        style: ProjectileStyle,
        damage: u32,
        origin: Vec2,
        velocity: Vec2,
    ) -> Self {
        Self { team, style, damage, origin, velocity, additional: () }
    }

}

impl<T: Bundle> SpawnProjectile<T> {

    #[must_use]
    pub fn with<A: Bundle>(self, components: A) -> SpawnProjectile<(T, A)> {
        let SpawnProjectile{ team, style, damage, origin, velocity, additional } = self;
        SpawnProjectile{
            team,
            style,
            damage,
            origin,
            velocity,
            additional: (additional, components)
        }
    }

}

impl<T: Bundle> Command for SpawnProjectile<T> {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world: &mut World, styles: Mut<ProjectileStyles>| {
            let style = styles.defs.get(&self.style).unwrap();

            let transform = Transform2D::new(self.origin, Vec2::ZERO);

            let render = PbrBundle{
                transform: Transform::from_translation(self.origin.extend(0.0)),
                mesh: style.mesh.clone(),
                material: match self.team {
                    Team::Player => style.material_player.clone(),
                    Team::Enemy  => style.material_enemy.clone(),
                },
                ..default()
            };

            let collider = DamageSourceOnce{
                shape:  style.shape,
                amount: self.damage,
                hit:    None,
            };

            let velocity = ProjectileLinear{
                velocity: self.velocity
            };

            match self.team {
                Team::Player => { world.spawn((TeamPlayer, render, transform, collider, velocity, self.additional, TransformSync)); },
                Team::Enemy  => { world.spawn((TeamEnemy,  render, transform, collider, velocity, self.additional, TransformSync)); },
            }

        });
    }
}