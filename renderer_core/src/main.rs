
use std::path::Path;

use renderer_core::mesh::mesh_loader::load_vao;
use renderer_core::world::camera::Camera;
use renderer_core::canvas::Canvas;
use renderer_core::util::Throwable;


fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
    let mut _canvas = Canvas::new(w, h)?;
    let mut _camera = Camera::new(aspect);
    
    load_vao(&Path::new("./resource/test_model.glb"))?;
    

    Ok(())
}
