use std::f64::consts::PI;

use rasterizer_rs::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Color, Stopwatch, Throwable, Vec4, Vec4Project};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
	let mut stopwatch = Stopwatch::new();
	stopwatch.start();
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    let n = 60;
    while canvas.update()? {
		for i in 0..n {
			let theta = -PI/2.0 + (i as f64) * 2.0*PI / (n as f64);
			let r = 1.0;
			let p1 = Vec4::new(0., 0., 0., 0.);
			let p2 = 
				&(r * &Vec4::new(f64::cos(theta), f64::sin(theta), 0., 0.)) + &p1;
			let p1 = Vec4Project(p1);
			let p2 = Vec4Project(p2);
			let red = (theta + PI/2.)/(2.*PI);
			camera.draw_line(&mut canvas, &p1, &p2, &Color::new(red, red, 1., 1.));
		}
	}
    Ok(())
}
