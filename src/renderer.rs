use minifb::Window;
use crate::camera::Camera;
use crate::draw::{execute, DrawCmd};
use crate::framebuffer::Framebuffer;
use crate::scene::Scene;

pub struct Renderer {
    framebuffer: Framebuffer,
    window: Window,
    camera: Camera,
    draw_cmds: Vec<DrawCmd>,
}

impl Renderer {
    pub fn new(framebuffer: Framebuffer, window: Window, camera: Camera) -> Self {
        Self {
            framebuffer,
            window,
            camera,
            draw_cmds: Vec::new(),
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        self.framebuffer.clear();
        self.draw_cmds.clear();

        for object in &scene.objects {
            object.draw_cmds(&mut self.draw_cmds);
        }

        for cmd in &self.draw_cmds {
            execute(cmd, &mut self.framebuffer, &self.camera);
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
