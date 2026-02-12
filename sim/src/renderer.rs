use crate::boat::Boat;
use crate::draw::Drawing;
use crate::environment::Environment;
use glam::Vec2;
use std::f32::consts::PI;

mod colors {
    pub const WIND: u32 = 0x446688;
    pub const BG: u32 = 0x203050;
    pub const WINDEX: u32 = 0xFFA500;
    pub const HULL: u32 = 0xAAAAAA;
    pub const SAIL: u32 = 0xFFFFFF;
    pub const RUDDER: u32 = 0x882222;
}

const GRID_STEP: f32 = 50.0;

const HULL_LENGTH: f32 = 30.0;
const HULL_WIDTH: f32 = 12.0;
const SAIL_LENGTH: f32 = 25.0;
const RUDDER_LENGTH: f32 = 10.0;

pub struct Renderer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Drawing for Renderer {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }
    fn buffer(&self) -> &[u32] {
        &self.buffer
    }
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn draw_scene(&mut self, boat: &Boat, env: &Environment) {
        let camera_pos = boat.pos;

        self.clear(colors::BG);

        let start_x = (camera_pos.x / GRID_STEP).floor() * GRID_STEP;
        let start_y = (camera_pos.y / GRID_STEP).floor() * GRID_STEP;

        for gx in -15..15 {
            for gy in -10..10 {
                let world_pos = Vec2::new(
                    start_x + gx as f32 * GRID_STEP,
                    start_y + gy as f32 * GRID_STEP,
                );

                if let Some(screen_p) = self.world_to_screen(world_pos, camera_pos) {
                    let wind = env.get_wind_at(world_pos);
                    self.draw_arrow_centered(
                        screen_p,
                        wind.y.atan2(wind.x),
                        ((wind.x) * (wind.x) + (wind.x) * (wind.x)).sqrt(),
                        colors::WIND,
                    );
                }
            }
        }

        if let Some(screen_pos) = self.world_to_screen(boat.pos, camera_pos) {
            self.draw_boat(boat, env, screen_pos);
        }
    }

    fn draw_boat(&mut self, boat: &Boat, env: &Environment, center: Vec2) {}
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
