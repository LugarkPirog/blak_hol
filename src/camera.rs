use crate::base_classes::Vec3;

/// Maps world coordinates (meters, XY plane, +Y up) to screen pixels.
pub struct Camera {
    pub center: Vec3,
    /// Pixels per meter
    pub scale: f32,
    pub width: usize,
    pub height: usize,
}

impl Camera {
    pub fn looking_at_xy_plane(
        center: Vec3,
        view_half_extent_m: f32,
        width: usize,
        height: usize,
    ) -> Self {
        let scale = (width.min(height) as f32 / 2.0) / view_half_extent_m;
        Self {
            center,
            scale,
            width,
            height,
        }
    }

    pub fn world_to_screen(&self, world: Vec3) -> (f32, f32) {
        (
            (world.x - self.center.x) * self.scale + self.width as f32 / 2.0,
            self.height as f32 / 2.0 - (world.y - self.center.y) * self.scale,
        )
    }

    pub fn screen_to_world(&self, x: usize, y: usize) -> Vec3 {
        Vec3::new(
            (x as f32 - self.width as f32 / 2.0) / self.scale + self.center.x,
            self.center.y - (y as f32 - self.height as f32 / 2.0) / self.scale,
            0.0,
        )
    }

    pub fn world_length_to_pixels(&self, length_m: f32) -> f32 {
        length_m * self.scale
    }
}
