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
    assert!(in_range(0, x, WIDTH));
    assert!(in_range(0, y, HEIGHT));
    let pixel_pos = y * WIDTH + x;
    buffer[pixel_pos] = encode_color(color);
}

/**
 * 点p1から点p2への線分をBresenhamの線分描画アルゴリズムによって描画する。
 */
fn draw_line(buffer: &mut Vec<u32>, p1:&Point2, p2:&Point2, color: &Color) {
    // どの軸に沿って線形走査を行うかを表す。`true`ならばx軸に沿って線を描き、`false`ならy軸に沿って線を描く。
    let along_x_axis = (p2.x - p1.x).abs() > (p2.y - p1.y).abs();
    // 走査する軸の方向に対して、開始点と到着点が真逆になっていたときはp1とp2を入れ替えておく。
    if along_x_axis && p1.x > p2.x || !along_x_axis && p1.y > p2.y  {
        return draw_line(buffer, p2, p1, color);
    }
    // |dy/dx| > 1であったときはy = xに対して対称に移動させる。点を打つ際にはちゃんと移動させた分を戻す。
    let flipped_points = (&p1.flipped(), &p2.flipped());
    let (p1, p2) = if along_x_axis { (p1, p2) } else { flipped_points };

    // Bresenhamのアルゴリズムの実装（ただし、|dy/dx| < 1で実行できるように拡張）
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    assert!(dy.abs() < dx.abs()); // |dy/dx| < 1
    // 誤差が大きくなった場合に描画する画素の対象を正の方向にずらすか負の方向にずらすか
    let s = if dy > 0 { 1 } else { -1 };
    let mut y = p1.y;
    let mut err = -dx;
    for x in p1.x..p2.x {
        err += 2 * i32::abs(dy);
        if err > 0 {
            y += s;
            err -= 2 * dx;
        }
        // 点を打つ際にはちゃんと対称に移動させた分を戻す。
        let (x, y) = if along_x_axis {(x, y)} else {(y, x)};
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
        for i in 0..50 {
            let theta = -PI/2.0 + (i as f64) * 2.0*PI / 50.0;
            let r = (HEIGHT as f64) / 2.0;
            let p1 = Point2::new((WIDTH as i32) / 2, (HEIGHT as i32) / 2);
            let p2 = Point2::new(
                (p1.x as f64 + r * f64::cos(theta)).floor() as i32,
                (p1.y as f64 + r * f64::sin(theta)).floor() as i32
            );
            draw_line(&mut buffer, &p1, &p2, &Color::new(1., 1., 1.));
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }
    Ok(())
}
