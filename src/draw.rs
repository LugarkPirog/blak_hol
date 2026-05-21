use crate::base_classes::{Color, Vec3};
use crate::camera::Camera;
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

pub fn execute(cmd: &DrawCmd, framebuffer: &mut Framebuffer, camera: &Camera) {
    match cmd {
        DrawCmd::FilledDisc {
            center,
            radius,
            color,
        } => draw_filled_disc(framebuffer, camera, *center, *radius, *color),
        DrawCmd::Point {
            pos,
            brightness,
            size,
        } => draw_point(framebuffer, camera, *pos, *brightness, *size),
        DrawCmd::GradientSegment {
            from,
            to,
            brightness_from,
            brightness_to,
            size,
        } => draw_gradient_segment(
            framebuffer,
            camera,
            *from,
            *to,
            *brightness_from,
            *brightness_to,
            *size,
        ),
    }
}

fn draw_filled_disc(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    center: Vec3,
    radius: f32,
    color: Color,
) {
    let (cx, cy) = camera.world_to_screen(center);
    let r_px = camera.world_length_to_pixels(radius);
    let r_px_i = r_px.ceil() as i32;

    for dy in -r_px_i..=r_px_i {
        for dx in -r_px_i..=r_px_i {
            if (dx * dx + dy * dy) as f32 > r_px * r_px {
                continue;
            }
            let x = cx + dx as f32;
            let y = cy + dy as f32;
            if x >= 0.0
                && y >= 0.0
                && (x as usize) < framebuffer.width
                && (y as usize) < framebuffer.height
            {
                framebuffer.set_pixel(x as usize, y as usize, color);
            }
        }
    }
}

fn draw_point(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    pos: Vec3,
    brightness: f32,
    size: i32,
) {
    let half = size as f32 / 2.0;
    let color = Color::white_scaled(brightness);
    let (base_x, base_y) = camera.world_to_screen(pos);

    for i in 0..size {
        for j in 0..size {
            let x = (base_x + i as f32 - half) as i32;
            let y = (base_y + j as f32 - half) as i32;
            if x >= 0
                && y >= 0
                && (x as usize) < framebuffer.width
                && (y as usize) < framebuffer.height
            {
                framebuffer.set_pixel(x as usize, y as usize, color);
            }
        }
    }
}

fn draw_gradient_segment(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    from: Vec3,
    to: Vec3,
    brightness_from: f32,
    brightness_to: f32,
    size: i32,
) {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let world_steps = dx.hypot(dy);
    let pixel_steps = camera.world_length_to_pixels(world_steps).ceil() as i32;
    let steps = pixel_steps.max(1);

    for s in 0..=steps {
        let t = s as f32 / steps as f32;
        let p = Vec3::new(from.x + dx * t, from.y + dy * t, 0.0);
        let brightness = brightness_from + (brightness_to - brightness_from) * t;
        draw_point(framebuffer, camera, p, brightness, size);
    }
}
