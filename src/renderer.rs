use minifb::Window;
use crate::draw::{execute, DrawCmd, Viewport};
use crate::framebuffer::Framebuffer;
use crate::scene::Scene;

pub struct Renderer {
    framebuffer: Framebuffer,
    window: Window,
    viewport: Viewport,
    draw_cmds: Vec<DrawCmd>,
}

impl Renderer {
    pub fn new(framebuffer: Framebuffer, window: Window) -> Self {
        let viewport = Viewport::new(framebuffer.width, framebuffer.height);
        Self {
            framebuffer,
            window,
            viewport,
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
            execute(cmd, &mut self.framebuffer, &self.viewport);
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
