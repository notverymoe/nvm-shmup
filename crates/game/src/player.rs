// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, color::palettes::css as Colors};

use crate::{BundleProjectile, DamageSink, TeamPlayer, Transform2D, TransformSync};

#[derive(Debug, Default, Bundle)]
pub struct PlayerBundle {
    pub input_config:  PlayerInputConfig,
    pub input:         PlayerInput,
    pub controller:    PlayerController,
    pub damage_sink:   DamageSink, 
    pub transform:     Transform2D,
    pub fire_cooldown: PlayerWeaponCooldown, 
    pub team:          TeamPlayer,
}

#[derive(Debug, Clone, Copy)]
pub enum InputButton {
    KeyCode(KeyCode),
    GamepadButton(GamepadButtonType),
    GamepadAnalog(GamepadAxisType, f32),
}

impl InputButton {

    #[must_use]
    pub fn pressed(
        &self, 
        button_kb: &ButtonInput<KeyCode>,
        button_gp: &ButtonInput<GamepadButton>,
        axis_gp:   &Axis<GamepadAxis>,
        gamepad:   Option<Gamepad>,
    ) -> bool {
        match self {
            Self::KeyCode(k) => button_kb.pressed(*k),
            Self::GamepadButton(k) => {
                button_gp.get_pressed()
                    .filter(|v| v.button_type == *k && (gamepad.is_none() || v.gamepad == gamepad.unwrap()))
                    .any(|v| button_gp.pressed(*v))
            }
            Self::GamepadAnalog(a, t) => {
                axis_gp.devices()
                    .filter(|v| v.axis_type == *a && (gamepad.is_none() || v.gamepad == gamepad.unwrap()))
                    .any(|v| axis_gp.get(*v).unwrap_or(0.0) > *t)
            }
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub enum InputAxis {
    Buttons([InputButton; 2], [f32; 2]),
    Axis(GamepadAxisType),
}

impl InputAxis {

    #[must_use]
    pub fn get(
        &self, 
        button_kb: &ButtonInput<KeyCode>,
        button_gp: &ButtonInput<GamepadButton>,
        axis_gp:   &Axis<GamepadAxis>,
        gamepad:   Option<Gamepad>,
    ) -> f32 {
        match self {
            Self::Buttons([a, b], [a_wt, b_wt]) => {
                  (if a.pressed(button_kb, button_gp, axis_gp, gamepad) { *a_wt } else { 0.0 })
                + (if b.pressed(button_kb, button_gp, axis_gp, gamepad) { *b_wt } else { 0.0 })
            },
            Self::Axis(a) => {
                normalize_axis(
                    axis_gp.devices()
                        .filter(|v| v.axis_type == *a && (gamepad.is_none() || v.gamepad == gamepad.unwrap()))
                        .map(|v| axis_gp.get(*v).unwrap_or(0.0))
                        .sum::<f32>()
                )
            }
        }
    }

}


#[derive(Debug, Component)]
pub struct PlayerInputConfig {
    pub force_digital: bool,
    pub gamepad: Option<Gamepad>,
    pub axis_x: Vec<InputAxis>,
    pub axis_y: Vec<InputAxis>,
    pub fire:   Vec<InputButton>,
}

impl Default for PlayerInputConfig {
    fn default() -> Self {
        Self { 
            force_digital: true,
            gamepad: None,
            axis_x: vec![
                InputAxis::Buttons([
                    InputButton::KeyCode(KeyCode::KeyD),
                    InputButton::KeyCode(KeyCode::KeyA),
                ], [1.0, -1.0]),
                InputAxis::Buttons([
                    InputButton::GamepadButton(GamepadButtonType::DPadRight),
                    InputButton::GamepadButton(GamepadButtonType::DPadLeft),
                ], [1.0, -1.0]),
                InputAxis::Axis(GamepadAxisType::LeftStickX),
            ],
            axis_y: vec![
                InputAxis::Buttons([
                    InputButton::KeyCode(KeyCode::KeyW),
                    InputButton::KeyCode(KeyCode::KeyS),
                ], [1.0, -1.0]),
                InputAxis::Buttons([
                    InputButton::GamepadButton(GamepadButtonType::DPadUp),
                    InputButton::GamepadButton(GamepadButtonType::DPadDown),
                ], [1.0, -1.0]),
                InputAxis::Axis(GamepadAxisType::LeftStickY),
            ],
            fire: vec![
                InputButton::KeyCode(KeyCode::Space),
                InputButton::GamepadButton(GamepadButtonType::East),
            ]
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct PlayerInput {
    pub move_dir: Vec2,
    pub fire:     bool,
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

pub fn prepare_player_input(mut q_player: Query<&mut PlayerInput>) {
    q_player.iter_mut().for_each(|mut player| player.move_dir = Vec2::ZERO);
}

pub fn update_keyboard_input(
    mut q_player: Query<(&PlayerInputConfig, &mut PlayerInput)>,
    button_kb: Res<ButtonInput<KeyCode>>,
    button_gp: Res<ButtonInput<GamepadButton>>,
    axis_gp: Res<Axis<GamepadAxis>>,
) {
    q_player.iter_mut().for_each(|(config, mut player)| {
        let move_dir = Vec2::new(
            config.axis_x.iter().map(|a| a.get(&button_kb, &button_gp, &axis_gp, config.gamepad)).sum::<f32>(),
            config.axis_y.iter().map(|a| a.get(&button_kb, &button_gp, &axis_gp, config.gamepad)).sum::<f32>()
        );
        player.move_dir += if config.force_digital { axes_digital(move_dir, 0.2) } else { move_dir };
        player.fire = config.fire.iter().any(|v| v.pressed(&button_kb, &button_gp, &axis_gp, config.gamepad));
    });
}

fn axes_digital(dir: Vec2, deadzone: f32) -> Vec2 {
    if dir.length_squared() <= deadzone*deadzone {
        Vec2::ZERO
    } else {
        Vec2::from_angle((dir.to_angle()/core::f32::consts::FRAC_PI_4).round()*core::f32::consts::FRAC_PI_4)
    }
}

fn normalize_axis(amount: f32) -> f32 {
    amount.abs().min(1.0).max(0.0) * amount.signum()
}

pub fn update_player_movement(
    mut q_player: Query<(&mut Transform2D, &PlayerController, &PlayerInput)>,
    time: Res<Time>,
) {
    q_player.iter_mut().for_each(|(mut transform, controller, input)| {
        if let Some(move_dir) = input.move_dir.try_normalize() {
            transform.position.current += move_dir * controller.move_speed * time.delta_seconds();
        }
    });
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PlayerWeaponCooldown {
    pub accum: f32,
}

pub fn update_player_firing(
    mut q_player: Query<(&mut PlayerWeaponCooldown, &Transform2D, &PlayerInput)>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    q_player.iter_mut().for_each(|(mut cooldown, transform, input)| {
        if cooldown.accum > 0.0 {
            cooldown.accum = (cooldown.accum - time.delta_seconds()).max(0.0);
            if cooldown.accum > 0.0 {
                return;
            }
        }

        if input.fire {
            cooldown.accum = 0.1;
            commands.spawn((
                BundleProjectile::bullet(TeamPlayer, transform.position.current, Vec2::Y * 100.0, 0.25, 1),
                PbrBundle { // TODO improve on this
                    mesh: meshes.add(Sphere::new(0.125)),
                    transform: Transform::from_translation(transform.position.current.extend(0.0)),
                    material: materials.add(Color::from(Colors::BLUE)),
                    ..default()
                },
                TransformSync
            ));
        }
    });
}

pub struct PluginPlayer;

impl Plugin for PluginPlayer {
    fn build(&self, app: &mut App) {
        app
            .add_systems(First,  prepare_player_input)
            .add_systems(Update, (update_keyboard_input, update_player_movement, update_player_firing).chain());
    }
}
