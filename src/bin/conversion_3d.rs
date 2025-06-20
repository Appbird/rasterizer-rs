use std::f32::consts::PI;
use std::{thread, time};

use rasterizer_rs::actor::Actor;
use rasterizer_rs::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Stopwatch, Throwable, Vec4};

pub fn conversion_3d(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let mut stopwatch = Stopwatch::new();
	stopwatch.start();
    let mut previous_instant = 0.0;
    let blue = Vec4::newvec(0.2, 0.2, 0.9);
    let green = Vec4::newvec(0.2, 0.9, 0.2);
	let red = Vec4::newvec(0.9, 0.2, 0.2);
    let world:Vec<Actor> = vec![
        Actor{
            vertices: [Vec4::newvec(0., 0., 0.), Vec4::newvec(3., 1., 2.)/5., Vec4::newvec(0., -3., 4.)/5.],
            position: Vec4::newvec(0., 0., 0.),
            axis: Vec4::newvec(0., 0., 0.),
            theta: 0.0,
            color: [blue.clone(), green.clone(), red.clone()]
        },
        Actor{
            vertices: [Vec4::newvec(0., 4., -5.)/5., Vec4::newvec(-3., -1., -2.)/5., Vec4::newvec(0., 3., 4.)/5.],
            position: Vec4::newvec(0., 0., 0.),
            axis: Vec4::newvec(0., 0., 0.),
            theta: 0.0,
            color: [green.clone(), blue.clone(), blue.clone()]
        }
    ];
    while canvas.update()? {
		let theta = PI/6;
		camera.location = Vec4{f64::cos()};
		
        let current_instant = stopwatch.elapsed_as_sec();
        let delta_time = (current_instant - previous_instant) / 1000.;
        previous_instant = current_instant;
        let _fps = 1.0 / delta_time;
        camera.snapshot(canvas, &world);
        
        let waiting_time = time::Duration::from_millis(30);
        thread::sleep(waiting_time);
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
