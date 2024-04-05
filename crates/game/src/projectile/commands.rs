// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{ecs::world::Command, prelude::*};

use crate::{prelude::ProjectileAim, tags::prelude::*, transform::prelude::*};

use super::{plugin::{ProjectileDamage, ProjectileSpeed}, styles::{ProjectileStyle, ProjectileStyles}};

#[derive(Debug, Clone, Copy)]
pub enum Team {
    Player,
    Enemy
}
pub trait CommandsSpawnProjectile {
    fn spawn_projectile(&mut self, team: Team, style: ProjectileStyle, damage: f32, aim: ProjectileAim);
    fn spawn_projectile_with<T: Bundle>(&mut self, team: Team, style: ProjectileStyle, damage: f32, aim: ProjectileAim, bundle: T);
}

impl<'w, 's> CommandsSpawnProjectile for Commands<'w, 's> {
    fn spawn_projectile(&mut self, team: Team, style: ProjectileStyle, damage: f32, aim: ProjectileAim) {
        self.add(SpawnProjectile::new(team, style, damage, aim));
    }

    fn spawn_projectile_with<T: Bundle>(&mut self, team: Team, style: ProjectileStyle, damage: f32, aim: ProjectileAim, bundle: T) {
        self.add(SpawnProjectile::new(team, style, damage, aim).with(bundle));
    }
}

pub struct SpawnProjectile<T: Bundle = ()> {
    pub team:       Team,
    pub style:      ProjectileStyle,
    pub damage:     f32,
    pub aim:        ProjectileAim,
    pub additional: T,
}

impl SpawnProjectile<()> {

    #[must_use]
    pub const fn new(
        team:   Team,
        style:  ProjectileStyle,
        damage: f32,
        aim:    ProjectileAim,
    ) -> Self {
        Self { team, style, damage, aim, additional: () }
    }

}

impl<T: Bundle> SpawnProjectile<T> {

    #[must_use]
    pub fn with<A: Bundle>(self, components: A) -> SpawnProjectile<(T, A)> {
        let SpawnProjectile{ team, style, damage, aim, additional } = self;
        SpawnProjectile{
            team,
            style,
            damage,
            aim,
            additional: (additional, components)
        }
    }

}

impl<T: Bundle> Command for SpawnProjectile<T> {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world: &mut World, styles: Mut<ProjectileStyles>| {
            let style = styles.defs.get(&self.style).unwrap();

            let transform: Transform2D = self.aim.into();

            let render = PbrBundle{
                transform: Transform::from_translation(transform.position.current.extend(0.0)),
                mesh: style.mesh.clone(),
                material: match self.team {
                    Team::Player => style.material_player.clone(),
                    Team::Enemy  => style.material_enemy.clone(),
                },
                ..default()
            };

            let collider = ProjectileDamage{
                shape:  style.shape,
                amount: self.damage,
            };

            let speed = ProjectileSpeed(self.aim.speed);

            match self.team {
                Team::Player => { world.spawn((TeamPlayer, render, transform, collider, speed, self.additional, TransformSync)); },
                Team::Enemy  => { world.spawn((TeamEnemy,  render, transform, collider, speed, self.additional, TransformSync)); },
            }

        });
    }
}
