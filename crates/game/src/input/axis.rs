// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::prelude::*;

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

