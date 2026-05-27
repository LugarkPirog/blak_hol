use std::collections::VecDeque;

use crate::base_classes::{Color, PolarCoord, Vec3};
use crate::constants::C;
use crate::draw::DrawCmd;

const RAY_TRAIL_LEN: usize = 30;

pub enum Absorber {
    BlackHole(BlackHole),
}

impl Absorber {
    pub fn contains(&self, point: Vec3) -> bool {
        match self {
            Absorber::BlackHole(black_hole) => black_hole.contains(point),
        }
    }

    pub fn draw_cmds(&self, out: &mut Vec<DrawCmd>) {
        match self {
            Absorber::BlackHole(black_hole) => black_hole.draw_cmds(out),
        }
    }
}

impl From<BlackHole> for Absorber {
    fn from(black_hole: BlackHole) -> Self {
        Absorber::BlackHole(black_hole)
    }
}

pub struct BlackHole {
    pub center: Vec3,
    pub radius: f32,
    pub mass: f32,
}

impl BlackHole {
    pub fn new(center: Vec3, radius: f32, mass: f32) -> Self {
        Self {
            center,
            radius,
            mass,
        }
    }

    pub fn contains(&self, point: Vec3) -> bool {
        (point.x - self.center.x).powf(2.0)
            + (point.y - self.center.y).powf(2.0)
            + (point.z - self.center.z).powf(2.0)
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
    pub r: f32,
    pub phi: f32,
    pub dr: f32,
    pub dphi: f32,
    pub draw_size: i32,
    trail: VecDeque<Vec3>,
    absorbed: bool,
}

impl Ray {
    pub fn new(pos: Vec3, direction: Vec3) -> Self {
        let pos_polar = PolarCoord::from(pos);
        let r = pos_polar.radius;
        let phi = pos_polar.phi;
        let dir_len = direction.length();
        let (dr, dphi) = if dir_len > 0.0 {
            let nx = direction.x / dir_len;
            let ny = direction.y / dir_len;
            let dr = nx * phi.cos() + ny * phi.sin();
            let dphi = - phi.sin() / r;
            (dr, dphi)
        } else {
            (0.0, 0.0)
        };

        let mut trail = VecDeque::with_capacity(RAY_TRAIL_LEN);
        trail.push_back(pos);
        Self {
            r,
            phi,
            dr,
            dphi,
            draw_size: 1,
            trail,
            absorbed: false,
        }
    }

    pub fn step(&mut self, dl: f32) {
        let dt = C * dl;
        self.r += self.dr * dt;
        self.phi += self.dphi * dt;

        self.trail.push_back(self.pos());
        while self.trail.len() > RAY_TRAIL_LEN {
            self.trail.pop_front();
        }
    }

    pub fn pos(&self) -> Vec3 {
        Vec3::new(
            self.r * self.phi.cos(),
            self.r * self.phi.sin(),
            0.0,
        )
    }

    pub fn draw_cmds(&self, out: &mut Vec<DrawCmd>) {
        let pos = self.pos();
        let n = self.trail.len();
        if n == 0 {
            out.push(DrawCmd::Point {
                pos,
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
            to: pos,
            brightness_from: trail_brightness(n - 1, n),
            brightness_to: 1.0,
            size: self.draw_size,
        });
    }

    pub fn is_absorbed(&self) -> bool {
        self.absorbed
    }

    pub fn mark_as_absorbed(&mut self) {
        self.absorbed = true;
    }
}

fn trail_brightness(index: usize, len: usize) -> f32 {
    if len <= 1 {
        return 1.0;
    }
    (index + 1) as f32 / len as f32
}
