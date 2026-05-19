use crate::scene::Scene;

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn step(&mut self, scene: &mut Scene, dt: f32) {
        for object in &mut scene.objects {
            object.step(dt);
        }
    }
}
