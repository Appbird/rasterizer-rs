use std::rc::Rc;

use image::{ImageReader, Rgba32FImage};

use crate::{actor::Actor, canvas::Canvas, component::{MeshRenderer, Transform}, shader::{pipeline::RenderingPipeline, texture_pipeline::TexturePipeline}, util::{Mat4x4, Vec4, Vec4Model, VertexArrayObject}, world::camera::Camera};

type TextureUniform = <TexturePipeline as RenderingPipeline>::Uniform;
type TextureAttribute = <TexturePipeline as RenderingPipeline>::Attribute;

pub fn plane_mesh() -> VertexArrayObject<TextureAttribute> {
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
    VertexArrayObject::<_> {
        attribute: vectors.into_iter().zip(uv).map(|(e, v)| TextureAttribute{ point: Vec4Model(e), uv:v }).collect(),
        idx: indecies
    }
}


#[derive(Clone)]
pub struct PlaneActor {
    pub mesh:VertexArrayObject<TextureAttribute>,
    pub transform:Transform,
    pub renderer: MeshRenderer<TexturePipeline>,
    pub texture:Rc<Rgba32FImage>
    
}

impl PlaneActor {
    pub fn new() -> Self {
        let texture = ImageReader::open("resource/texture.jpg").unwrap().decode().unwrap();
        let texture = Rc::new(texture.to_rgba32f());
        Self {
            mesh: plane_mesh(),
            transform: Transform::new(),
            renderer: MeshRenderer::new(TexturePipeline{} ),
            texture
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
    fn render(&self, _camera:&Camera, canvas:&mut Canvas, pv:&Mat4x4) -> () {
        let uni = TextureUniform{
            pvm: pv * &self.transform.model_conversion(),
            size: canvas.size(),
            texture: self.texture.clone(),
        };
        self.renderer.render(canvas, &self.mesh, &uni);
    }
    fn is_terminated(&self) -> bool {
        false
    }
}