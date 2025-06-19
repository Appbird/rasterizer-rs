use rasterizer_rs::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Color, Throwable, Vec4, Vec4Project};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    
    
    let points = [
        Vec4::new(0.5, 0.2, 0., 0.),
        Vec4::new(-0.8, -0.5, 0., 0.),
        Vec4::new(-0.5, 0.5, 0., 0.)
    ];
    let points:Vec<Vec4Project> = points.iter().map(|e| Vec4Project(e.clone())).collect();
    let white = Color::new(1., 1., 1., 1.);
    let red = Color::new(1., 0., 0., 1.);
    for point in &points {
        camera.draw_point(&mut canvas, &point, &red);
    }
	while canvas.update()? {
		camera.draw_triangle(
			&mut canvas,
			&points[0], &points[1], &points[2],
			&white
		);
	}
    Ok(())
}
