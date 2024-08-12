use minifb::{Key, Window, WindowOptions};
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Vec3 {
    e:[f64;3]
}
impl Vec3 {
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 { Vec3{ e:[x, y, z] } }
}
type Color = Vec3;

fn encode_color(color:Color) -> u32 {
    let r = (color.x() * 255.999).floor() as u32;
    let g = (color.y() * 255.999).floor() as u32;
    let b = (color.z() * 255.999).floor() as u32;
    return (r << 8 * 2) | (g << 8 * 1) | (b << 8 * 0); 
}
fn draw_pixel(buffer:&mut Vec<u32>, x:usize, y:usize, color:Color) {
    let pixel_pos = y * WIDTH + x;
    buffer[pixel_pos] = encode_color(color);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "rasterizer-rs",
        WIDTH, HEIGHT,
        WindowOptions::default(),
    )?;

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..30 {
            draw_pixel(&mut buffer, i * WIDTH / 30  , HEIGHT / 2, Color::new(1., 1., 1.));
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }
    Ok(())
}
