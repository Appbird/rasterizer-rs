use crate::{canvas::Canvas, util::{Mat4x4, Vec4, Vec4Model}, world::{actor::Actor, camera::Camera, mesh::TexturedMesh, textured_mesh_renderer::TextureMeshRenderer, transform::Transform}};

pub fn plane_mesh() -> TexturedMesh {
    let vectors = vec![
        Vec4::newpoint(0., -1., -1.),
        Vec4::newpoint(0., -1., 1.),
        Vec4::newpoint(0., 1., 1.),
        Vec4::newpoint(0., 1., -1.),
        
    ];
    let indecies: Vec<[usize; 3]> = vec![
        [0, 1, 2],
        [2, 3, 0],
    ];
    let uv: Vec<Vec4> = vec![
        Vec4::newpoint(0., 0., 0.),
        Vec4::newpoint(0., 1., 0.),
        Vec4::newpoint(1., 1., 0.),
        Vec4::newpoint(1., 0., 0.),
    ];
    TexturedMesh {
        vertices: vectors.into_iter().map(|e| { Vec4Model(e) }).collect(),
        uv,
        v_idx: indecies
    }
}


#[derive(Clone)]
pub struct PlaneActor {
    pub mesh:TexturedMesh,
    pub transform:Transform,
    pub mesh_renderer: TextureMeshRenderer,
}

impl PlaneActor {
    pub fn new() -> Self {
        Self {
            mesh: plane_mesh(),
            transform: Transform::new(),
            mesh_renderer: TextureMeshRenderer::new()
        }
    }
}

impl Actor for PlaneActor {
    fn update(&mut self, dt:f64) -> () {
        self.transform.update(dt);
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn render(&self, camera:&Camera, canvas:&mut Canvas, pv:&Mat4x4) -> () {
        self.mesh_renderer.render(&self.mesh, camera, canvas, pv, &self.transform.model_conversion());
    }
    fn is_terminated(&self) -> bool {
        false
    }
}