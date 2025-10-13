use minifb::{Window, WindowOptions, Key};
use crate::util::{in_range, Point2, Stopwatch, Vec4};
use crate::util::Color;

pub struct Canvas {
    window:Window,
    pub width:usize,
    pub height:usize,
    pub background_color:Vec4,
    color_buffer:Vec<u32>,
	depth_buffer:Vec<f64>,
    from_start:Stopwatch,
    from_prev_frame:Stopwatch,
    deltatime: f64,
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
        let color_buffer = vec![0; width * height];
		let depth_buffer = vec![1.; width * height];
        let background_color = Vec4::new(0.06, 0.07, 0.07, 1.0);
        let mut from_start = Stopwatch::new();
        let from_prev_frame = Stopwatch::new();
        from_start.start();
        let mut canvas = Canvas{
            window, width, height,
            color_buffer, depth_buffer,
            background_color,
            from_prev_frame, from_start,
            deltatime: 0.
        };
        canvas.window.set_target_fps(60);
        Ok(canvas)
    }
    pub fn draw_pixel(&mut self, p: &Point2, color: &Color) {
        let w = self.width as i32;
        let h = self.height as i32;
        if in_range(0, p.x, w) && in_range(0, p.y, h) {
            let pixel_pos = p.y * w + p.x;
            self.color_buffer[pixel_pos as usize] = encode_color(color);
        }
    }
	pub fn draw_pixel_with_depth(&mut self, p: &Point2, depth:&f64, color: &Color) {
        let w = self.width as i32;
        let h = self.height as i32;
		let pixel_pos = (p.y * w + p.x) as usize;
        if in_range(0, p.x, w)
		&& in_range(0, p.y, h)
		&& &self.depth_buffer[pixel_pos] > depth{
			self.color_buffer[pixel_pos] = encode_color(&color);
			self.depth_buffer[pixel_pos] = *depth;
        }
    }
	pub fn draw_depth(&mut self, p: &Point2, depth:&f64) {
        let w = self.width as i32;
        let h = self.height as i32;
		let pixel_pos = (p.y * w + p.x) as usize;
        if in_range(0, p.x, w)
		&& in_range(0, p.y, h)
		&& &self.depth_buffer[pixel_pos] > depth{
			let x = (depth + 1.)/2.;
			let x2 = x*x;
			let x4 = x2*x2;
			let x8 = x4*x4;
			let x16 = x8*x8;
			let x32 = x16*x16;
			let color_depth = 1. - x32;
			self.color_buffer[pixel_pos] = encode_color(&Color::new(color_depth, color_depth, color_depth, 0.0));
			self.depth_buffer[pixel_pos] = *depth;
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
    pub fn passed_time(&self) -> f64 {
        self.from_start.elapsed_as_sec()
    }
    pub fn deltatime(&self) -> f64 {
        self.deltatime
    }
    pub fn update(&mut self) -> minifb::Result<bool> {
        self.deltatime = self.from_prev_frame.elapsed_as_sec();
        self.from_prev_frame.reset();  
        self.from_prev_frame.start();
        self.window.update_with_buffer(&self.color_buffer, self.width, self.height)?;
        for i in 0 .. self.width * self.height {
            self.color_buffer[i] = encode_color(&self.background_color);
			self.depth_buffer[i] = 1.;
        }
        Ok(self.window.is_open() && !self.window.is_key_down(Key::Escape))
    }
    pub fn size(&self) -> Point2 {
        Point2::new(self.width as i32, self.height as i32)
    }
}