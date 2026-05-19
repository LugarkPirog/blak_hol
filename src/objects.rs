use std::collections::VecDeque;

use crate::base_classes::{Color, Vec3};
use crate::framebuffer::Framebuffer;

const RAY_TRAIL_LEN: usize = 30;

pub enum Object {
    BlackHole(BlackHole),
    Ray(Ray),
}

impl Object {
    pub fn step(&mut self, dt: f32) {
        match self {
            Object::BlackHole(_) => {}
            Object::Ray(ray) => ray.step(dt),
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        match self {
            Object::BlackHole(black_hole) => black_hole.draw(framebuffer),
            Object::Ray(ray) => ray.draw(framebuffer),
        }
    }
}

impl From<BlackHole> for Object {
    fn from(black_hole: BlackHole) -> Self {
        Object::BlackHole(black_hole)
    }
}

impl From<Ray> for Object {
    fn from(ray: Ray) -> Self {
        Object::Ray(ray)
    }
}

pub struct BlackHole {
    pub center: Vec3,
    pub radius: f32,
}

impl BlackHole {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn contains(&self, pixel: Vec3) -> bool {
        (pixel.x - self.center.x).powf(2.0)
            + (pixel.y - self.center.y).powf(2.0)
            + (pixel.z - self.center.z).powf(2.0)
            <= self.radius.powf(2.0)
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                let pixel = Vec3::new(
                    x as f32 - framebuffer.width as f32 / 2.0,
                    framebuffer.height as f32 / 2.0 - y as f32,
                    0.0,
                );
                if self.contains(pixel) {
                    framebuffer.set_pixel(x, y, Color::new(255, 0, 0));
                }
            }
        }
    }
}

pub struct Ray {
    pub pos: Vec3,
    pub direction: Vec3,
    pub draw_size: i32,
    trail: VecDeque<Vec3>,
}

impl Ray {
    pub fn new(pos: Vec3, direction: Vec3) -> Self {
        let mut trail = VecDeque::with_capacity(RAY_TRAIL_LEN);
        trail.push_back(pos);
        Self {
            pos,
            direction,
            draw_size: 1,
            trail,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.pos += self.direction * dt;
        self.trail.push_back(self.pos);
        while self.trail.len() > RAY_TRAIL_LEN {
            self.trail.pop_front();
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        let n = self.trail.len();
        if n == 0 {
            self.draw_point(framebuffer, self.pos, 1.0);
            return;
        }

        for i in 0..n - 1 {
            let b0 = trail_brightness(i, n);
            let b1 = trail_brightness(i + 1, n);
            self.draw_segment(framebuffer, self.trail[i], self.trail[i + 1], b0, b1);
        }

        let tail_brightness = trail_brightness(n - 1, n);
        self.draw_segment(framebuffer, self.trail[n - 1], self.pos, tail_brightness, 1.0);
    }

    fn draw_segment(
        &self,
        framebuffer: &mut Framebuffer,
        from: Vec3,
        to: Vec3,
        brightness_from: f32,
        brightness_to: f32,
    ) {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let steps = dx.hypot(dy).ceil() as i32;
        let steps = steps.max(1);

        for s in 0..=steps {
            let t = s as f32 / steps as f32;
            let p = Vec3::new(from.x + dx * t, from.y + dy * t, 0.0);
            let brightness = brightness_from + (brightness_to - brightness_from) * t;
            self.draw_point(framebuffer, p, brightness);
        }
    }

    fn draw_point(&self, framebuffer: &mut Framebuffer, pos: Vec3, brightness: f32) {
        let size = self.draw_size;
        let half = size as f32 / 2.0;
        let color = Color::white_scaled(brightness);

        for i in 0..size {
            for j in 0..size {
                let x = (pos.x + i as f32 - half - framebuffer.width as f32 / 2.0) as usize;
                let y = (pos.y + j as f32 - half - framebuffer.height as f32 / 2.0) as usize;
                if x < framebuffer.width && y < framebuffer.height {
                    framebuffer.set_pixel(x, y, color);
                }
            }
        }
    }
}

fn trail_brightness(index: usize, len: usize) -> f32 {
    if len <= 1 {
        return 1.0;
    }
    // 0 = oldest (dim), len-1 = newest point in trail (bright)
    (index + 1) as f32 / len as f32
}
