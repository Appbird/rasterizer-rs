use std::f64::consts::PI;
use  std::collections::VecDeque;

use rand::Rng;
use rasterizer_rs::world::{self, actor};
use rasterizer_rs::world::actor::Actor;
use rasterizer_rs::world::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{Stopwatch, Throwable, Vec4};
use rasterizer_rs::world::sample_model::tetrahedron;
use rand::seq::IndexedRandom;


pub fn conversion_3d(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let mut from_start = Stopwatch::new();
    let mut from_prev_frame = Stopwatch::new();
	from_start.start();
    
	let k = 2.*PI / 10.;
    let r = 15.0;
    let red = Vec4::newvec(0.8, 0.2, 0.2);
    let green = Vec4::newvec(0.2, 0.8, 0.2);
    let blue = Vec4::newvec(0.2, 0.2, 0.8);
    let tetras = [
        tetrahedron([red.clone(), red.clone(), blue.clone()]),
        tetrahedron([green.clone(), green.clone(), blue.clone()]),
        tetrahedron([red.clone(), blue.clone(), green.clone()])
    ];
    
    let mut rng = rand::rng();

    let mut world = vec![];
    let mut to_be_destroyed = Vec::<usize>::new(); 
    
    let mut count_tetra = 0;
    let tetra_interval = 0.13;
    while canvas.update()? {
        let t = from_start.elapsed_as_sec();
        let dt = from_prev_frame.elapsed_as_sec();
        from_prev_frame.reset();
        from_prev_frame.start();
		
        camera.position = Vec4::newpoint(r*f64::cos(k*t), r*f64::sin(k*t), 0.) ;
		camera.look = Vec4::newvec(-f64::cos(k*t), -f64::sin(k*t), 0.) ;
		camera.up = Vec4::newvec(0., 0., 1.);
        
        if t > tetra_interval * (count_tetra as f64) {
            count_tetra += 1;
            let tetra = tetras.choose(&mut rng).unwrap().clone();
            let mut actor1 = Actor::new(tetra);
            let theta_fire = 2.*PI / 3. * t;
            actor1.velocity = Vec4::newvec(f64::cos(theta_fire), f64::sin(theta_fire), 1.).normalized3d() * 15.;
            actor1.acc =  Vec4::newvec(0., 0., -12.);
            actor1.theta = Vec4::choice_in_sphere(&mut rng) * PI;
            actor1.omega = Vec4::choice_in_sphere(&mut rng) * rng.random_range(0. .. 4.*PI);
            let mut actor2 = actor1.clone();
            actor2.velocity = -actor2.velocity;
            world.push(actor1); world.push(actor2);
        }
        for (idx, actor) in world.iter_mut().enumerate() {
            actor.update(dt);
            if actor.is_terminated() { to_be_destroyed.push(idx); }
        }
        while let Some(idx) = to_be_destroyed.pop() {
            world.swap_remove(idx);
        }
        

        // DONE: Actorに速度・加速度をつける！
        // deltatime, Actorへの機能追加
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
