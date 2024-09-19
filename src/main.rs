mod util;
mod canvas;
mod camera;
use std::f64::consts::PI;

use camera::Camera;
use canvas::Canvas;
use util::{Color, Throwable, Vec4, Vec4Project};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new();
    
    while canvas.update()? {
        for i in 0..50 {
            let theta = -PI/2.0 + (i as f64) * 2.0*PI / 50.0;
            let r = 1.0;
            let p1 = Vec4::new(0., 0., 0., 0.);
            let p2 = 
                &(r * &Vec4::new(f64::cos(theta), f64::sin(theta), 0., 0.)) + &p1;
            let p1 = Vec4Project(p1);
            let p2 = Vec4Project(p2);
            camera.draw_line(&mut canvas, &p1, &p2, &Color::new(1., 1., 1., 1.));
        }
    }
    Ok(())
}
