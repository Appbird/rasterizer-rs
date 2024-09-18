mod util;
mod canvas;
use std::f64::consts::PI;

use canvas::Canvas;
use util::{Throwable, Color, Point2};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;


fn main() -> Throwable<()> {
    let mut canvas = Canvas::new(WIDTH, HEIGHT)?;

    while canvas.update()? {
        for i in 0..50 {
            let theta = -PI/2.0 + (i as f64) * 2.0*PI / 50.0;
            let r = (HEIGHT as f64) / 2.0;
            let p1 = Point2::new((WIDTH as i32) / 2, (HEIGHT as i32) / 2);
            let p2 = Point2::new(
                (p1.x as f64 + r * f64::cos(theta)).floor() as i32,
                (p1.y as f64 + r * f64::sin(theta)).floor() as i32
            );
            canvas.draw_line(&p1, &p2, &Color::new(1., 1., 1.));
        }
    }
    Ok(())
}
