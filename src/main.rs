use minifb::{Window, WindowOptions};
use std::thread::sleep;
use std::time::Duration;

mod base_classes;
mod camera;
mod constants;
mod draw;
mod objects;
mod framebuffer;
mod scene;
mod engine;
mod renderer;

use crate::base_classes::{PolarCoord, Vec3};
use crate::camera::Camera;
use crate::constants::{
    BLACK_HOLE_MASS, BLACK_HOLE_RADIUS, SIM_DT, VIEW_RADIUS_MULT,
};
use crate::objects::{BlackHole, Ray};
use crate::framebuffer::Framebuffer;
use crate::scene::Scene;
use crate::engine::Engine;
use crate::renderer::Renderer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const N_RAYS: usize = 20;

fn main() {
    let polar = PolarCoord::from(Vec3::new(1.0, 1.0, 0.0));
    println!("polar: {:?}", polar);
    let view_half_extent = BLACK_HOLE_RADIUS * VIEW_RADIUS_MULT;
    let camera = Camera::looking_at_xy_plane(
        Vec3::new(0.0, 0.0, 0.0),
        view_half_extent,
        WIDTH,
        HEIGHT,
    );

    let framebuffer = Framebuffer::new(HEIGHT, WIDTH);
    let window = Window::new("Black Hole", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut scene = Scene::new();
    let mut engine = Engine::new();
    let mut renderer = Renderer::new(framebuffer, window, camera);

    scene.add_absorber(BlackHole::new(
        Vec3::new(0.0, 0.0, 0.0),
        BLACK_HOLE_RADIUS,
        BLACK_HOLE_MASS,
    ));

    // Rays enter from the left in world meters (+X toward the black hole at origin)
    let spawn_x = -view_half_extent * 0.85;
    let spawn_y_span = view_half_extent * 0.75;
    for i in 0..N_RAYS {
        let t = i as f32 / (N_RAYS - 1) as f32;
        let y = -spawn_y_span + t * 2.0 * spawn_y_span;
        scene.add_ray(Ray::new(
            Vec3::new(spawn_x, y, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ));
    }

    loop {
        engine.step(&mut scene, SIM_DT);
        renderer.render(&scene);
        sleep(Duration::from_millis(10));
    }
}
