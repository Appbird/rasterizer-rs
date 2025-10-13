use std::f64::consts::PI;

use rasterizer_rs::world::actor::{Actor, Mesh};
use rasterizer_rs::world::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Stopwatch, Throwable, Vec4};



fn prepare_world() -> Vec<Actor> {
    let blue = Vec4::newvec(0.2, 0.2, 0.9);
    let green = Vec4::newvec(0.2, 0.9, 0.2);
	let red = Vec4::newvec(0.9, 0.2, 0.2);
    
    let polygons = vec![
        Mesh{
            vertices:[Vec4::newpoint(1., 2., 3.), Vec4::newpoint(1., -2., 0.), Vec4::newpoint(-1., -1., -3.)],
            color: [red.clone(), blue.clone(), red.clone()]  
        },
        Mesh{
            vertices:[Vec4::newpoint(1., 2., -3.), Vec4::newpoint(-1., -1., 3.), Vec4::newpoint(1., -2., 0.)],
            color: [green.clone(), blue.clone(), green.clone()]  
        },
    ];
    
    vec![
        Actor{
            polygons: polygons.clone(),
            position: Vec4::newpoint(2., 0., 0.),
            axis: Vec4::newvec(-1., 0., 0.),
            theta: PI/6.,
            scale: Vec4::newpoint(1., 1., 1.),
        },
        Actor{
            polygons: polygons.clone(),
            position: Vec4::newpoint(2., -3., 0.),
            axis: Vec4::newvec(0., 1., 1.).normalized3d(),
            theta: PI/6.,
            scale: Vec4::newpoint(1., 1., 1.),
        },
        Actor{
            polygons: polygons.clone(),
            position: Vec4::newpoint(2., 3., 0.),
            axis: Vec4::newvec(0., 0., 1.),
            theta: 3.*PI/2.,
            scale: Vec4::newpoint(1., 1., 1.),
        }
    ]
}

pub fn conversion_3d(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let mut stopwatch = Stopwatch::new();
	stopwatch.start();
    
	let k = 2.*PI / 5.;
    let world = prepare_world();
    while canvas.update()? {
		/*
        	let delta_time = (t - previous_instant) / 1000.;
        	previous_instant = t;
	        let _fps = 1.0 / delta_time;
		*/
        let t = stopwatch.elapsed_as_sec();
		camera.position = Vec4::newpoint(8.*f64::cos(k*t), 8.*f64::sin(k*t), 0.) ;
		camera.look = Vec4::newvec(-f64::cos(k*t), -f64::sin(k*t), 0.) ;
		camera.up = Vec4::newvec(0., 0., 1.) ;
		
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
