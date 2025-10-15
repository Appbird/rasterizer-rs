use std::f64::consts::PI;

use renderer_core::world::camera::Camera;
use renderer_core::canvas::Canvas;
use renderer_core::util::{ease, Throwable, Vec4};
use renderer_core::actor::plane::PlaneActor;
use renderer_core::world::world::World;


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
