use std::f64::consts::PI;

use rasterizer_rs::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Color, Mat4x4, Stopwatch, Throwable, Vec4, Vec4Project};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
	let mut stopwatch = Stopwatch::new();
	stopwatch.start();
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    let points = [
        Vec4::new(0.5, 0.25, 0., 1.),
        Vec4::new(0.5, 0.00, 0., 1.),
        Vec4::new(0.0, 0.00, 0., 1.),
        Vec4::new(0.0, 0.25, 0., 1.),
    ];
    while canvas.update()? {
		let t = stopwatch.elapsed_as_sec();
		let points:Vec<Vec4Project> = points.iter().map(|e| {
			let m =
				Mat4x4::translate(&Vec4::newvec( -0.5, -0.25, 0.))
				* Mat4x4::rotation(&Vec4::newvec(0., 0., 1.), PI*t/6.)
				* Mat4x4::scale(&Vec4::newvec(0.8, 0.5, 1.));
			Vec4Project(m * e)
		}).collect();

		let white = Color::new(1., 1., 1., 1.);
		for point in &points {
			camera.draw_point(&mut canvas, point, &white);
		}
		for (i, j) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
			let p1 = &points[i];
			let p2 = &points[j];
			camera.draw_line(&mut canvas, p1, p2, &white);
		}
	}
	Ok(())
}

