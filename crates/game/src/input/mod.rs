// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

mod button;
pub use button::*;

mod axis;
pub use axis::*;

mod util;
pub use util::*;

#[derive(Debug, Clone, Copy)]
pub struct UnifiedInput<'a> {
    pub button_kb: &'a ButtonInput<KeyCode>,
    pub button_gp: &'a ButtonInput<GamepadButton>,
    pub axis_gp:   &'a Axis<GamepadAxis>,
    pub gamepad:   Option<Gamepad>,
}

impl<'a> UnifiedInput<'a> {

    #[must_use]
    pub const fn new(
        button_kb: &'a ButtonInput<KeyCode>,
        button_gp: &'a ButtonInput<GamepadButton>,
        axis_gp:   &'a Axis<GamepadAxis>,
        gamepad:   Option<Gamepad>,
    ) -> Self {
        Self{button_kb, button_gp, axis_gp, gamepad}
    }

    #[must_use]
    pub const fn with_gamepad(self, gamepad: Option<Gamepad>) -> Self {
        Self{ gamepad, ..self }
    }

    #[must_use]
    pub fn pressed(&self, button: InputButton) -> bool {
        button.pressed(self.button_kb, self.button_gp, self.axis_gp, self.gamepad)
    }

    #[must_use]
    pub fn pressed_any<'b>(&self, buttons: impl IntoIterator<Item = &'b InputButton>) -> bool {
        buttons.into_iter().any(|button| self.pressed(*button))
    }

    #[must_use]
    pub fn axis(&self, axis: InputAxis) -> f32 {
        axis.get(self.button_kb, self.button_gp, self.axis_gp, self.gamepad)
    }

    #[must_use]
    pub fn axis_2(&self, axis_x: InputAxis, axis_y: InputAxis) -> Vec2 {
        Vec2::new(self.axis(axis_x), self.axis(axis_y)).normalize_or_zero()
    }

    #[must_use]
    pub fn axes<'b>(&self, axes: impl IntoIterator<Item = &'b InputAxis>) -> f32 {
        axes.into_iter().map(|axis| self.axis(*axis)).sum::<f32>().max(-1.0).min(1.0)
    }

    #[must_use]
    pub fn axes_2<'b>(&self, axes_x: impl IntoIterator<Item = &'b InputAxis>, axes_y: impl IntoIterator<Item = &'b InputAxis>) -> Vec2 {
        Vec2::new(self.axes(axes_x), self.axes(axes_y)).normalize_or_zero()
    }
}
