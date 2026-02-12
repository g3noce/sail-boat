use crate::environment::Environment;
use glam::Vec2;
use minifb::{Key, Window};
use std::f32::consts::PI;

const MAX_RUDDER_ANGLE: f32 = 45.0 * PI / 180.0;
const RUDDER_RESPONSE_SPEED: f32 = 2.0;

pub struct Boat {
    pub pos: Vec2,
    pub vel: Vec2,
    pub heading: f32,
    pub rudder_angle: f32,
    pub sail_aperture: f32,
    pub sail_angle: f32,
}

impl Boat {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            heading: 0.0,
            rudder_angle: 0.0,
            sail_aperture: 0.1,
            sail_angle: 0.0,
        }
    }

    pub fn handle_input(&mut self, window: &Window, dt: f32) {
        let target_rudder = if window.is_key_down(Key::Left) {
            MAX_RUDDER_ANGLE
        } else if window.is_key_down(Key::Right) {
            -MAX_RUDDER_ANGLE
        } else {
            0.0
        };

        self.rudder_angle += (target_rudder - self.rudder_angle) * RUDDER_RESPONSE_SPEED * dt;

        if window.is_key_down(Key::Down) {
            self.sail_aperture = (self.sail_aperture - 1.0 * dt).max(0.05);
        }
        if window.is_key_down(Key::Up) {
            self.sail_aperture = (self.sail_aperture + 1.0 * dt).min(PI / 2.0);
        }
    }

    pub fn update(&mut self, dt: f32, env: &Environment) {}
}

pub fn normalize_angle(angle: f32) -> f32 {
    let mut a = angle;
    while a > PI {
        a -= 2.0 * PI;
    }
    while a < -PI {
        a += 2.0 * PI;
    }
    a
}
