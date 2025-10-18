
use std::f64::consts::PI;
use std::path::Path;

use renderer_core::actor::gltf_test::GltfTestActor;
use renderer_core::mesh::mesh_loader::load_vao;
use renderer_core::world::camera::Camera;
use renderer_core::canvas::Canvas;
use renderer_core::util::{Throwable, Vec4};
use renderer_core::world::world::World;


fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    
    let mut world = World::<GltfTestActor>::new();
    let vao_seq = load_vao(&Path::new("./resource/test_model.glb"))?;
    
    vao_seq.into_iter()
    .map(|vao| GltfTestActor::new(vao))
    .for_each(|actor| world.spawn(actor));
    
    while canvas.update()? {
        let t = canvas.passed_time();
        let theta = 2.*PI/5. * t;
        let r = 3.;
        camera.position = Vec4::newpoint(r* f64::cos(theta), r*f64::sin(theta), 1.);
        camera.look = -camera.position.normalized3d();
        camera.up = Vec4::newvec(0., 0., 1.);

        world.update(canvas.deltatime());
        world.draw(&camera, &mut canvas);
    }

    Ok(())
}
