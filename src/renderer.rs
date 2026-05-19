use minifb::Window;
use crate::framebuffer::Framebuffer;
use crate::scene::Scene;

pub struct Renderer {
    framebuffer: Framebuffer,
    window: Window,
}

impl Renderer {
    pub fn new(framebuffer: Framebuffer, window: Window) -> Self {
        Self { framebuffer, window }
    }

    pub fn render(&mut self, scene: &Scene) {
        self.framebuffer.clear();
        for object in &scene.objects {
            object.draw(&mut self.framebuffer);
        }
        self.window
            .update_with_buffer(
                &self.framebuffer.pixels,
                self.framebuffer.width,
                self.framebuffer.height,
            )
            .unwrap();
    }
}
