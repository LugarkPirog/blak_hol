use crate::base_classes::Color;

pub struct Framebuffer {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width, pixels: vec![0; height * width] }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn clear_with(&mut self, color: Color) {
        self.pixels.fill(color.to_u32());
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * self.width + x] = color.to_u32();
    }
}