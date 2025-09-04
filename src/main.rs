use std::f64::consts::PI;

use rasterizer_rs::world::actor::{Actor, Polygon};
use rasterizer_rs::world::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Stopwatch, Throwable, Vec4};
use rasterizer_rs::world::sample_model::tetrahedron;
use rand::seq::{IndexedRandom, SliceRandom};


pub fn conversion_3d(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let mut stopwatch = Stopwatch::new();
	stopwatch.start();
    
	let k = 2.*PI / 5.;
    let red = Vec4::newvec(0.8, 0.2, 0.2);
    let green = Vec4::newvec(0.2, 0.8, 0.2);
    let blue = Vec4::newvec(0.2, 0.2, 0.8);
    let tetras = [
        tetrahedron([red.clone(), red.clone(), blue.clone()]),
        tetrahedron([green.clone(), green.clone(), blue.clone()]),
        tetrahedron([blue.clone(), blue.clone(), green.clone()])
    ];
    
    let rng = rand::rng();
    let world = vec![];
    while canvas.update()? {
        let t = stopwatch.elapsed_as_sec();
		camera.position = Vec4::newpoint(3.*f64::cos(k*t), 3.*f64::sin(k*t), 0.) ;
		camera.look = Vec4::newvec(-f64::cos(k*t), -f64::sin(k*t), 0.) ;
		camera.up = Vec4::newvec(0., 0., 1.) ;
        let tetra = tetras.choose(&mut rng).unwrap().clone();
        world.clear();
        world.push(Actor{tetra});
        // TODO: Actorに速度・加速度をつける！
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
