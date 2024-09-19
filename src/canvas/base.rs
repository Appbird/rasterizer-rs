use minifb::{Window, WindowOptions, Key};
use crate::util::in_range;
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
    pub fn draw_pixel(&mut self, x: usize, y:usize, color: &Color) {
        assert!(in_range(0, x, self.width));
        assert!(in_range(0, y, self.height));
        let pixel_pos = y * self.width + x;
        self.buffer[pixel_pos] = encode_color(color);
    }
    pub fn update(&mut self) -> minifb::Result<bool> {
        self.window.update_with_buffer(&self.buffer, self.width, self.height)?;
        Ok(self.window.is_open() && !self.window.is_key_down(Key::Escape))
    }
}