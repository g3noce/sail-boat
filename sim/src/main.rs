mod boat;
mod draw;
mod environment;
mod renderer;

use crate::boat::Boat;
use crate::draw::Drawing;
use crate::environment::Environment;
use crate::renderer::Renderer;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut window = Window::new(
        "Simulateur Focus V3 - ESC pour quitter",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    let mut boat = Boat::new(0.0, 0.0);
    let mut env = Environment::new();
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let dt = 1.0 / 60.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        boat.handle_input(&window, dt);

        env.update(dt);
        boat.update(dt, &env);

        renderer.draw_scene(&boat, &env);

        window
            .update_with_buffer(renderer.buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
