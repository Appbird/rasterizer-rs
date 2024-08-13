use minifb::{Key, Window, WindowOptions};
use std::f64::consts::PI;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Point2 {
    x: i32,
    y: i32,
}
impl Point2 {
    pub fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }
    pub fn flipped(&self) -> Self {
        Point2::new(self.y, self.x)
    }
}
struct Vec3 {
    e: [f64; 3],
}
impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }
}
type Color = Vec3;

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


fn in_range<T: PartialOrd>(begin: T, x: T, end: T) -> bool {
    begin <= x && x < end
}

fn draw_pixel(buffer: &mut Vec<u32>, x: usize, y: usize, color: &Color) {
    assert!(in_range(0, x, WIDTH) && in_range(0, y, HEIGHT));
    let pixel_pos = y * WIDTH + x;
    buffer[pixel_pos] = encode_color(color);
}

/**
 * 点p1から点p2への線分をBresenhamの線分描画アルゴリズムによって描画する。
 */
fn draw_line(buffer: &mut Vec<u32>, p1:&Point2, p2:&Point2, color: &Color) {
    if p1.x > p2.x { draw_line(buffer, p2, p1, color); }

    let based_on_x_axis = (p2.x - p1.x).abs() > (p2.y - p1.y).abs();
    let flipped_points = (&p1.flipped(), &p2.flipped());
    let (p1, p2) = if based_on_x_axis { (p1, p2) } else { flipped_points };
    
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let mut y = p1.y;
    let mut err = -dx;
    for x in p1.x..p2.x {
        err += 2 * dy;
        if err > 0 {
            y += 1;
            err -= 2*dx;
        }
        let (x, y) = if based_on_x_axis {(x, y)} else {(y, x)};
        if in_range(0, x, WIDTH as i32) && in_range(0, y, HEIGHT as i32) {
            draw_pixel(buffer, x as usize, y as usize, color)
        }
    }
    
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("rasterizer-rs", WIDTH, HEIGHT, WindowOptions::default())?;

    window.set_target_fps(24);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..8 {
            
            //draw_line(&mut buffer, &Point2::new(50, 50), &Point2::new(300, 300), &Color::new(1.0, 1.0, 1.0));
            draw_line(&mut buffer, &Point2::new(300, 50), &Point2::new(50, 300), &Color::new(1.0, 1.0, 1.0));
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }
    Ok(())
}
