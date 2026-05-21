use crate::objects::{Absorber, Ray};

pub struct Scene {
    pub absorbers: Vec<Absorber>,
    pub rays: Vec<Ray>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            absorbers: vec![],
            rays: vec![],
        }
    }

    pub fn add_absorber(&mut self, absorber: impl Into<Absorber>) {
        self.absorbers.push(absorber.into());
    }

    pub fn add_ray(&mut self, ray: Ray) {
        self.rays.push(ray);
    }
}
