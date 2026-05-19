use minifb::{Window, WindowOptions};
use std::thread::sleep;
use std::time::Duration;

mod base_classes;
mod objects;
mod framebuffer;
mod scene;
mod engine;
mod renderer;

use crate::base_classes::Vec3;
use crate::objects::{BlackHole, Ray};
use crate::framebuffer::Framebuffer;
use crate::scene::Scene;
use crate::engine::Engine;
use crate::renderer::Renderer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const N_RAYS: usize = 20;

const c: f32 = 299792458.0;
const dt: f32 = 5.0; // microseconds

fn main() {
    let framebuffer = Framebuffer::new(HEIGHT, WIDTH);
    let window = Window::new("Black Hole", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut scene = Scene::new();
    let mut engine = Engine::new();
    let mut renderer = Renderer::new(framebuffer, window);

    scene.add_object(BlackHole::new(Vec3::new(0.0, 0.0, 0.0), 100.0));

    let left_x = (WIDTH as f32) / 2.0;
    for i in 0..N_RAYS {
        let y = {
            let t = i as f32 / (N_RAYS - 1) as f32;
            t * HEIGHT as f32 + HEIGHT as f32 / 2.0
        };
        scene.add_object(Ray::new(
            Vec3::new(left_x, y, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ));
    }

    loop {
        engine.step(&mut scene, dt);
        renderer.render(&scene);
        sleep(Duration::from_millis(10));
    }
}
