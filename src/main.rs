use std::fs::File;
use std::io::{BufWriter, Write};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::thread::sleep;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }
}

struct Vec3 {
    x:f32,
    y:f32,
    z:f32,
}

impl Vec3 {
    fn new(x:f32, y:f32, z:f32) -> Self {
        Self { x, y, z }
    }
}
struct Circle {
    center: Vec3,
    radius: f32,
}

impl Circle {
    fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
    fn contains(&self, pixel: Vec3) -> Result<bool, String> {
        Ok((pixel.x - self.center.x).powf(2.0) + (pixel.y - self.center.y).powf(2.0) + (pixel.z - self.center.z).powf(2.0) <= self.radius.powf(2.0))
    }
}

fn main() {
    println!("Hello, world!");
    let mut pix = vec![0u32; WIDTH * HEIGHT];
    let mut window = Window::new("Black Hole", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut cnt = 0;

    let hole = Circle::new(Vec3::new(0.0, 0.0, 0.0), 100.0);
    'main_loop: loop {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = Vec3::new(
                    x as f32 - WIDTH as f32 / 2.0,
                    HEIGHT as f32 / 2.0 - y as f32,
                    0.0,
                );
                if hole.contains(pixel).expect("Failed to calculate distance") {
                    pix[y * WIDTH + x] = Color::new(255, 0, 0).to_u32();
                }
            }
        }
        window.update_with_buffer(&pix, WIDTH, HEIGHT).unwrap();
        sleep(Duration::from_millis(100));
    }
}
