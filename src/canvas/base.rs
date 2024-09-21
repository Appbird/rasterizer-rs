use minifb::{Window, WindowOptions, Key};
use crate::util::{in_range, Point2};
use crate::Color;

pub struct Canvas {
    window:Window,
    pub width:usize,
    pub height:usize,
    buffer:Vec<u32>
}

fn encode_color(color: &Color) -> u32 {
    let (r, g, b) = (
        color.x().clamp(0., 1.),
        color.y().clamp(0., 1.),
        color.z().clamp(0., 1.)
    );
    let (r, g, b) = (
        (r * 255.999).floor() as u32,
        (g * 255.999).floor() as u32,
        (b * 255.999).floor() as u32
    );
    return (r << 8 * 2) | (g << 8 * 1) | (b << 8 * 0);
}

impl Canvas {
    pub fn new(width:usize, height:usize) -> minifb::Result<Canvas>{
        let window = Window::new("rasterizer-rs", width, height, WindowOptions::default())?;
        let buffer = vec![0; width * height];
        let mut canvas = Canvas{window, width, height, buffer};
        canvas.window.set_target_fps(24);
        Ok(canvas)
    }
    pub fn draw_pixel(&mut self, p: &Point2, color: &Color) {
        let w = self.width as i32;
        let h = self.height as i32;
        if in_range(0, p.x, w) && in_range(0, p.y, h) {
            let pixel_pos = p.y * w + p.x;
            self.buffer[pixel_pos as usize] = encode_color(color);
        }
    }
    pub fn draw_point(&mut self, center:&Point2, color: &Color) {
        let half_width = 3;
        for x in -half_width .. half_width {
            for y in -half_width .. half_width {
                if x*x + y*y < half_width*half_width {
                    self.draw_pixel(&Point2::new(center.x + x, center.y + y), color);
                }
            }
        }
    }
    pub fn update(&mut self) -> minifb::Result<bool> {
        self.window.update_with_buffer(&self.buffer, self.width, self.height)?;
        Ok(self.window.is_open() && !self.window.is_key_down(Key::Escape))
    }
    pub fn size(&self) -> Point2 {
        Point2::new(self.width as i32, self.height as i32)
    }
}