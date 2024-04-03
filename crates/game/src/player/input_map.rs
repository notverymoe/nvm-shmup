// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::{input::PlayerInput, input_axis::InputAxis, input_button::InputButton, util::axes_digital};

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