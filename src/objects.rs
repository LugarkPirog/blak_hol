use std::collections::VecDeque;

use crate::base_classes::{Color, Vec3};
use crate::constants::C;
use crate::draw::DrawCmd;

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

    pub fn draw_cmds(&self, out: &mut Vec<DrawCmd>) {
        match self {
            Object::BlackHole(black_hole) => black_hole.draw_cmds(out),
            Object::Ray(ray) => ray.draw_cmds(out),
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
    pub mass: f32,
}

impl BlackHole {
    pub fn new(center: Vec3, radius: f32, mass: f32) -> Self {
        Self { center, radius, mass }
    }

    pub fn contains(&self, pixel: Vec3) -> bool {
        (pixel.x - self.center.x).powf(2.0)
            + (pixel.y - self.center.y).powf(2.0)
            + (pixel.z - self.center.z).powf(2.0)
            <= self.radius.powf(2.0)
    }

    pub fn draw_cmds(&self, out: &mut Vec<DrawCmd>) {
        out.push(DrawCmd::FilledDisc {
            center: self.center,
            radius: self.radius,
            color: Color::new(255, 0, 0),
        });
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
        let dir_len = (self.direction.x * self.direction.x
            + self.direction.y * self.direction.y
            + self.direction.z * self.direction.z)
            .sqrt();
        if dir_len > 0.0 {
            self.pos += self.direction * (C * dt / dir_len);
        }
        self.trail.push_back(self.pos);
        while self.trail.len() > RAY_TRAIL_LEN {
            self.trail.pop_front();
        }
    }

    pub fn draw_cmds(&self, out: &mut Vec<DrawCmd>) {
        let n = self.trail.len();
        if n == 0 {
            out.push(DrawCmd::Point {
                pos: self.pos,
                brightness: 1.0,
                size: self.draw_size,
            });
            return;
        }
        for i in 0..n - 1 {
            out.push(DrawCmd::GradientSegment {
                from: self.trail[i],
                to: self.trail[i + 1],
                brightness_from: trail_brightness(i, n),
                brightness_to: trail_brightness(i + 1, n),
                size: self.draw_size,
            });
        }
        out.push(DrawCmd::GradientSegment {
            from: self.trail[n - 1],
            to: self.pos,
            brightness_from: trail_brightness(n - 1, n),
            brightness_to: 1.0,
            size: self.draw_size,
        });
    }
}


fn trail_brightness(index: usize, len: usize) -> f32 {
    if len <= 1 {
        return 1.0;
    }
    (index + 1) as f32 / len as f32
}
