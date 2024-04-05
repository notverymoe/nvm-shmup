// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{input::prelude::*, PlayerController};

#[derive(Debug, Component)]
pub struct PlayerInputConfig {
    pub digital_deadzone: Option<f32>,
    pub gamepad: Option<Gamepad>,
    pub axis_x: Vec<InputAxis>,
    pub axis_y: Vec<InputAxis>,
    pub fire:   Vec<InputButton>,
}

impl Default for PlayerInputConfig {
    fn default() -> Self {
        Self { 
            digital_deadzone: Some(0.2),
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

pub fn update_player_input(
    mut q_player: Query<(&PlayerInputConfig, &mut PlayerController)>,
    button_kb: Res<ButtonInput<KeyCode>>,
    button_gp: Res<ButtonInput<GamepadButton>>,
    axis_gp: Res<Axis<GamepadAxis>>,
) {
    q_player.iter_mut().for_each(|(config, mut player)| {
        let input = UnifiedInput::new(&button_kb, &button_gp, &axis_gp, config.gamepad);
        player.move_dir += apply_digital(input.axes_2(&config.axis_x, &config.axis_y), config.digital_deadzone);
        player.fire = input.pressed_any(&config.fire);
    });
}

fn apply_digital(value: Vec2, deadzone: Option<f32>) -> Vec2 {
    if let Some(deadzone) = deadzone {
        axes_digital(value, deadzone)
    } else {
        value
    }
}