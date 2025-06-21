use std::f64::consts::PI;

use rasterizer_rs::actor::Actor;
use rasterizer_rs::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Stopwatch, Throwable, Vec4};

pub fn conversion_3d(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let mut stopwatch = Stopwatch::new();
	stopwatch.start();
    let blue = Vec4::newvec(0.2, 0.2, 0.9);
    let green = Vec4::newvec(0.2, 0.9, 0.2);
	let red = Vec4::newvec(0.9, 0.2, 0.2);
    let world:Vec<Actor> = vec![
        Actor{
            vertices: [Vec4::newpoint(0., 0., 0.), Vec4::newpoint(3., 1., -2.), Vec4::newpoint(0., -3., -4.)],
            position: Vec4::newpoint(0., 0., 0.),
            axis: Vec4::newvec(0., 0., 0.),
            theta: 0.0,
            color: [blue.clone(), green.clone(), red.clone()]
        },
        Actor{
            vertices: [Vec4::newpoint(0., 4., -5.), Vec4::newpoint(-3., -1., 2.), Vec4::newpoint(0., 3., -4.)],
            position: Vec4::newpoint(0., 0., 0.),
            axis: Vec4::newvec(0., 0., 0.),
            theta: 0.0,
            color: [green.clone(), blue.clone(), blue.clone()]
        }
    ];
	let k = 2.*PI / 5.;
    while canvas.update()? {
		/*
        	let delta_time = (t - previous_instant) / 1000.;
        	previous_instant = t;
	        let _fps = 1.0 / delta_time;
		*/
        let t = stopwatch.elapsed_as_sec();
		camera.look = Vec4::newpoint(6.*f64::sin(k*t), 0., 6.*f64::cos(k*t)) ;
		
        camera.snapshot(canvas, &world);
    }
    Ok(())
}

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    conversion_3d(&mut camera, &mut canvas)?;
    Ok(())
}
