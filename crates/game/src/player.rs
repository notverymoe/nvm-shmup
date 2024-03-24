// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Default, Bundle)]
pub struct PlayerBundle {
    pub keyboard:   PlayerKeyboardConfig,
    pub input:      PlayerInput,
    pub controller: PlayerController,
    pub collider:   Collider,
}

#[derive(Debug, Component)]
pub struct PlayerKeyboardConfig {
    pub key_n: KeyCode,
    pub key_e: KeyCode,
    pub key_s: KeyCode,
    pub key_w: KeyCode,
}

impl Default for PlayerKeyboardConfig {
    fn default() -> Self {
        Self { 
            key_n: KeyCode::KeyW, 
            key_e: KeyCode::KeyD, 
            key_s: KeyCode::KeyS, 
            key_w: KeyCode::KeyA, 
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct PlayerInput {
    pub move_dir: Vec2,
}

#[derive(Debug, Component)]
pub struct PlayerController {
    pub move_speed: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self { 
            move_speed: 20.0,
        }
    }
}

#[derive(Debug, Component)]
pub struct Collider {
    pub position: Vec2,
    pub bounds: Bounds,
}

impl Default for Collider {
    fn default() -> Self {
        Self { 
            position: Vec2::ZERO,
            bounds: Bounds::Circle(0.5),
        }
    }
}

#[derive(Debug)]
pub enum Bounds {
    Circle(f32),
    BoxAligned(Vec2),
    BoxOriented(Vec2, Vec2),
}

pub fn prepare_player_input(mut q_player: Query<&mut PlayerInput>) {
    q_player.iter_mut().for_each(|mut player| player.move_dir = Vec2::ZERO);
}

pub fn update_keyboard_input(
    mut q_player: Query<(&PlayerKeyboardConfig, &mut PlayerInput)>,
    input_kb: Res<ButtonInput<KeyCode>>,
) {
    q_player.iter_mut().for_each(|(config, mut player)| {
        if input_kb.pressed(config.key_n) { player.move_dir += Vec2::Y; }
        if input_kb.pressed(config.key_e) { player.move_dir += Vec2::X; }
        if input_kb.pressed(config.key_s) { player.move_dir -= Vec2::Y; }
        if input_kb.pressed(config.key_w) { player.move_dir -= Vec2::X; }
    });
}

pub fn update_player_movement(
    mut q_player: Query<(&mut Collider, &PlayerController, &PlayerInput)>,
    time: Res<Time>,
) {
    q_player.iter_mut().for_each(|(mut collider, controller, input)| {
        if let Some(move_dir) = input.move_dir.try_normalize() {
            collider.position += move_dir * controller.move_speed * time.delta_seconds();
        }
    });
}

pub struct PluginPlayer;

impl Plugin for PluginPlayer {
    fn build(&self, app: &mut App) {
        app
            .add_systems(First,  prepare_player_input)
            .add_systems(Update, (update_keyboard_input, update_player_movement).chain());
    }
}
