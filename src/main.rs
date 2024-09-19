mod util;
mod canvas;
mod camera;
use std::f64::consts::PI;

use canvas::Canvas;
use util::{Throwable, Color, Point2};

fn main() -> Throwable<()> {
    let W: usize = 640;
    let H: usize = 480;
    let mut canvas = Canvas::new(W, H)?;
    
    while canvas.update()? {
        for i in 0..50 {
            let theta = -PI/2.0 + (i as f64) * 2.0*PI / 50.0;
            let r = (H as f64) / 2.0;
            let p1 = Point2::new((W as i32) / 2, (H as i32) / 2);
            let p2 = Point2::new(
                (p1.x as f64 + r * f64::cos(theta)).floor() as i32,
                (p1.y as f64 + r * f64::sin(theta)).floor() as i32
            );
            canvas.draw_line(&p1, &p2, &Color::new(1., 1., 1., 1.));
        }
    }
    Ok(())
}
