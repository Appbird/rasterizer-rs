use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Throwable, Vec4, Vec4Project, Vec4Screen};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let mut canvas = Canvas::new(w, h)?;
    
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
    let points: [Vec4Screen; 3] =
		points
			.map(|e| Vec4Project::new(e)
			.into_screen(&canvas.size()));
	while canvas.update()? {
		canvas.draw_triangle(
			[points[0].clone(), points[1].clone(), points[2].clone()],
			&colors
		);
	}
    Ok(())
}
