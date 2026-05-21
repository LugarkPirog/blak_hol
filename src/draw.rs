use crate::base_classes::{Color, Vec3};
use crate::framebuffer::Framebuffer;

pub enum DrawCmd {
    FilledDisc {
        center: Vec3,
        radius: f32,
        color: Color,
    },
    Point {
        pos: Vec3,
        brightness: f32,
        size: i32,
    },
    GradientSegment {
        from: Vec3,
        to: Vec3,
        brightness_from: f32,
        brightness_to: f32,
        size: i32,
    },
}

pub struct Viewport {
    pub width: usize,
    pub height: usize,
}

impl Viewport {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn screen_to_world(&self, x: usize, y: usize) -> Vec3 {
        Vec3::new(
            x as f32 - self.width as f32 / 2.0,
            self.height as f32 / 2.0 - y as f32,
            0.0,
        )
    }

    fn ray_pos_to_screen(&self, pos: Vec3) -> (f32, f32) {
        (pos.x - self.width as f32 / 2.0, pos.y - self.height as f32 / 2.0)
    }
}

pub fn execute(cmd: &DrawCmd, framebuffer: &mut Framebuffer, viewport: &Viewport) {
    match cmd {
        DrawCmd::FilledDisc {
            center,
            radius,
            color,
        } => draw_filled_disc(framebuffer, viewport, *center, *radius, *color),
        DrawCmd::Point {
            pos,
            brightness,
            size,
        } => draw_point(framebuffer, viewport, *pos, *brightness, *size),
        DrawCmd::GradientSegment {
            from,
            to,
            brightness_from,
            brightness_to,
            size,
        } => draw_gradient_segment(
            framebuffer,
            viewport,
            *from,
            *to,
            *brightness_from,
            *brightness_to,
            *size,
        ),
    }
}

fn contains_disc(center: Vec3, radius: f32, pixel: Vec3) -> bool {
    (pixel.x - center.x).powf(2.0)
        + (pixel.y - center.y).powf(2.0)
        + (pixel.z - center.z).powf(2.0)
        <= radius.powf(2.0)
}

fn draw_filled_disc(
    framebuffer: &mut Framebuffer,
    viewport: &Viewport,
    center: Vec3,
    radius: f32,
    color: Color,
) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let pixel = viewport.screen_to_world(x, y);
            if contains_disc(center, radius, pixel) {
                framebuffer.set_pixel(x, y, color);
            }
        }
    }
}

fn draw_point(
    framebuffer: &mut Framebuffer,
    viewport: &Viewport,
    pos: Vec3,
    brightness: f32,
    size: i32,
) {
    let half = size as f32 / 2.0;
    let color = Color::white_scaled(brightness);
    let (base_x, base_y) = viewport.ray_pos_to_screen(pos);

    for i in 0..size {
        for j in 0..size {
            let x = (base_x + i as f32 - half) as usize;
            let y = (base_y + j as f32 - half) as usize;
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_pixel(x, y, color);
            }
        }
    }
}

fn draw_gradient_segment(
    framebuffer: &mut Framebuffer,
    viewport: &Viewport,
    from: Vec3,
    to: Vec3,
    brightness_from: f32,
    brightness_to: f32,
    size: i32,
) {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let steps = dx.hypot(dy).ceil() as i32;
    let steps = steps.max(1);

    for s in 0..=steps {
        let t = s as f32 / steps as f32;
        let p = Vec3::new(from.x + dx * t, from.y + dy * t, 0.0);
        let brightness = brightness_from + (brightness_to - brightness_from) * t;
        draw_point(framebuffer, viewport, p, brightness, size);
    }
}
