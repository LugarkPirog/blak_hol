use crate::scene::Scene;

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn step(&mut self, scene: &mut Scene, dt: f32) {
        for ray in &mut scene.rays {
            if !ray.is_absorbed() {
                ray.step(dt);
            }
        }

        for ray in &mut scene.rays {
            if ray.is_absorbed() {
                continue;
            }

            let pos = ray.pos();
            if scene.absorbers.iter().any(|a| a.contains(pos)) {
                ray.mark_as_absorbed();
            }
        }
    }
}
