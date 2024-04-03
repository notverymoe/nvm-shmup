// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

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
