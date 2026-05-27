use std::ops::{Add, Mul, Sub, AddAssign, SubAssign};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn length(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        (x * x + y * y + z * z).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = *self - other;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Self::Output {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn white_scaled(brightness: f32) -> Self {
        let b = (brightness.clamp(0.0, 1.0) * 255.0) as u8;
        Self::new(b, b, b)
    }

    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PolarCoord {
    pub radius: f32,
    pub phi: f32,
    pub theta: f32,
}

impl PolarCoord {
    pub fn new(radius: f32, phi: f32, theta: f32) -> Self {
        Self { radius, phi, theta }
    }
}

impl From<Vec3> for PolarCoord {
    fn from(vec: Vec3) -> Self {
        let l = vec.length();
        let radius = l;
        let phi = vec.y.atan2(vec.x);
        let theta = (vec.z / l).acos();
        Self::new(radius, phi, theta)
    }
}
