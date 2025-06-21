use std::f64::consts::PI;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Color, Stopwatch, Throwable, Vec4, Vec4Project};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
	let mut stopwatch = Stopwatch::new();
	stopwatch.start();
    let mut canvas = Canvas::new(w, h)?;
    let n = 60;
    while canvas.update()? {
		for i in 0..n {
			let theta = -PI/2.0 + (i as f64) * 2.0*PI / (n as f64);
			let r = 1.0;
			let p1 = Vec4::new(0., 0., 0., 0.);
			let p2 = 
				&(r * &Vec4::new(f64::cos(theta), f64::sin(theta), 0., 0.)) + &p1;
			let p1 = Vec4Project::new(p1).into_screen(&canvas.size()).to_point2();
			let p2 = Vec4Project::new(p2).into_screen(&canvas.size()).to_point2();
			let red = (theta + PI/2.)/(2.*PI);
			canvas.draw_line(&p1, &p2, &Color::new(red, red, 1., 1.));
		}
	}
    Ok(())
}
