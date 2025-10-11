use std::f64::consts::PI;

use rand::Rng;
use rasterizer_rs::world::camera::Camera;
use rasterizer_rs::canvas::Canvas;
use rasterizer_rs::util::{color, ease, Throwable, Vec4};
use rasterizer_rs::world::plane::PlaneActor;
use rasterizer_rs::world::tetrahedron::{tetrahedron_mesh, TetrahedronActor};
use rasterizer_rs::world::world::World;


pub fn tetrahedron_world(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let mut world = World::new();
    
    let r = 15.0;
    let mut rng = rand::rng();
    let mut count_tetra = 0;
    let tetra_interval = 0.10;
    
    while canvas.update()? {
        let t = canvas.passed_time();
        let dt = canvas.deltatime();
		
        let camera_theta = 2.*PI * ease::outin_quart(t / 4. % 1.);
        camera.position = Vec4::newpoint(  r*f64::cos(camera_theta), r*f64::sin(camera_theta),  2. * (f64::cos(camera_theta) - 0.5)) ;
		camera.look = (-&camera.position).normalized3d();
		camera.up = Vec4::newvec(0., 0., 1.);
        
        
        if t > tetra_interval * (count_tetra as f64) {
            count_tetra += 1;
            let theta_fire = 2.*PI / 1.5 * t;
            let c1 = color::hsv2vec4(360. * theta_fire / (2.*PI), 0.8, 0.8);
            let c2 = color::hsv2vec4(360. * theta_fire / (2.*PI), 0.3, 0.9);
            let tetra = tetrahedron_mesh([c1.clone(), c2.clone(), c2.clone(), c2.clone()]);
            let mut actor1 = TetrahedronActor::new(tetra);
            actor1.transform.velocity = Vec4::newvec(1.2*f64::cos(theta_fire), 1.2*f64::sin(theta_fire), 0.75).normalized3d() * 17.5;
            actor1.transform.acc =  Vec4::newvec(0., 0., -12.);
            actor1.transform.theta = Vec4::choice_in_sphere(&mut rng) * PI;
            actor1.transform.omega = Vec4::choice_in_sphere(&mut rng) * rng.random_range(0. .. 4.*PI);
            let mut actor2 = actor1.clone();
            
            actor2.transform.velocity = -actor2.transform.velocity;
            world.spawn(actor1);
            world.spawn(actor2);
        }
        world.update(dt);
        world.draw(camera, canvas);
    }
    Ok(())
}

pub fn plane_world(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let mut world = World::new();    
    
    let r = 2.0;
    let actor = PlaneActor::new();
    world.spawn(actor);
    while canvas.update()? {
        let t = canvas.passed_time();
        let dt = canvas.deltatime();
        let camera_theta = 2.*PI * ease::outin_quart(t / 4. % 1.);
        camera.position = Vec4::newpoint(  r*f64::cos(camera_theta), r*f64::sin(camera_theta),  /*2. * (f64::cos(camera_theta) - 0.5)*/ 0.) ;
		camera.look = (-&camera.position).normalized3d();
		camera.up = Vec4::newvec(0., 0., 1.);
        world.update(dt);
        world.draw(camera, canvas);
    }
    Ok(())
}

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    plane_world(&mut camera, &mut canvas)?;
    Ok(())
}
