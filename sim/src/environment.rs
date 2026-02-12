use glam::Vec2;

pub struct Environment {
    pub time: f32,
}

impl Environment {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn get_wind_at(&self, _pos: Vec2) -> Vec2 {
        Vec2::new(10.0, 0.0)
    }
}
