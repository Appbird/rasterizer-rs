use rasterizer_rs::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Color, Throwable, Vec4, Vec4Project};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    
    
    let points: [Vec4; 3] = [
        Vec4::newpoint(0.5, 0.2, 0.),
        Vec4::newpoint(-0.8, -0.5, 0.),
        Vec4::newpoint(-0.5, 0.5, 0.)
    ];
	let colors: [Vec4; 3] = [
        Vec4::newvec(0.2, 0.2, 0.9),
        Vec4::newvec(0.2, 0.9, 0.2),
        Vec4::newvec(0.9, 0.2, 0.2),
    ];
    let points: [Vec4Project; 3] = points.iter().map(|e| Vec4Project(e.clone())).collect::<Vec<_>>().try_into().unwrap();
	while canvas.update()? {
		camera.draw_triangle(
			&mut canvas,
			&[points[0].clone(), points[1].clone(), points[2].clone()],
			&colors
		);
	}
    Ok(())
}
