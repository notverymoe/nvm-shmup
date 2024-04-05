// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{math::DVec2, prelude::*};

use crate::prelude::Transform2D;

#[derive(Debug, Clone, Copy)]
pub struct ProjectileAim {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub speed:     f32,
}

impl ProjectileAim {

    #[must_use]
    pub const fn new(origin: Vec2, direction: Vec2, speed: f32) -> Self {
        Self{origin, direction, speed}
    }

    #[must_use]
    pub fn aim_at(self, target: Vec2, arc: f32) -> Self {
        if let Some(dir) = (target - self.origin).try_normalize() {
            let aim_angle = self.direction.perp_dot(dir);
            let aim_angle = aim_angle.signum() * aim_angle.abs().min(arc.cos());
            let aim_angle = Vec2::new((1.0 - aim_angle*aim_angle).sqrt(), aim_angle);
            Self {
                origin:    self.origin,
                direction: self.direction.rotate(aim_angle),
                speed:     self.speed,
            }
        } else {
            self
        }
    }

    #[must_use]
    pub fn aim_predictive(self, target: Vec2, target_velocity: Vec2, max_t: f32, arc: f32) -> Self {

        let predicted = if let Some(t) = calculate_aim_toi(self.origin.into(), target.into(), self.speed.into(), target_velocity.into()) {
            target + target_velocity*(t as f32).min(max_t)
        } else {
            target + target_velocity * max_t
        };

        self.aim_at(predicted, arc)
    }

    /// Offsets the origin in the projectile's aim direction by the specified distance, for fan attacks.
    /// ### Warning
    /// - Do not use after `Self::aim_predictive`, it will cause the bullet to overshoot
    #[must_use]
    pub fn with_offset(self, distance: f32) -> Self {
        // TODO... this will throw-off predictive aim :|
        Self {
            origin:    self.origin + self.direction*distance,
            direction: self.direction,
            speed:     self.speed,
        }
    }


}

impl From<ProjectileAim> for Transform2D {
    fn from(value: ProjectileAim) -> Self {
        Transform2D::new(value.origin, value.direction)
    }
}

fn calculate_aim_toi(start_source: DVec2, start_target: DVec2, speed_source: f64, velocity_target: DVec2) -> Option<f64> {
    let a = velocity_target.length_squared() - speed_source.powi(2);
    let b = 2.0 * velocity_target.dot(start_target - start_source);
    let c = start_target.distance_squared(start_source);

    let d = b.powi(2) - 4.0*a*c;
    if d <= 0.0 {
        return None;
    }

    let d_sqrt = d.sqrt();
    let div    = 2.0 * a;
    let t0     = (-b + d_sqrt)/div;
    let t1     = (-b - d_sqrt)/div;

    match (t0 >= 0.0, t1 >= 0.0) {
        ( true,  true) => Some(t0.min(t1)),
        ( true, false) => Some(t0),
        (false,  true) => Some(t1),
        (false, false) => None, // Shouldn't occur, but, y'know
    }
}
