
use std::rc::Rc;

use image::{DynamicImage, Rgba32FImage};

use crate::{actor::Actor, canvas::Canvas, component::{MeshRenderer, Transform}, mesh::VertexArrayObject, shader::{texture_pipeline::TexturePipeline, pipeline::RenderingPipeline}, util::Mat4x4, world::camera::Camera};

type TextureUni = <TexturePipeline as RenderingPipeline>::Uniform;
type TextureAttr = <TexturePipeline as RenderingPipeline>::Attribute;


#[derive(Clone)]
pub struct GltfTestActor {
    pub mesh:VertexArrayObject<TextureAttr>,
    pub transform:Transform,
    pub mesh_renderer: MeshRenderer<TexturePipeline>,
    texture: Rc<Rgba32FImage>,
    terminated:bool,
}

impl GltfTestActor {
    pub fn new(
        mesh:VertexArrayObject<TextureAttr>,
        image: Rgba32FImage
    ) -> Self {
        let mut mesh_renderer = MeshRenderer::new(TexturePipeline{});
        let texture = Rc::new(image);
        mesh_renderer.culling(true);
        Self {
            mesh,
            transform: Transform::new(),
            mesh_renderer,
            terminated: false,
            texture
        }
    }
}

impl Actor for GltfTestActor {
    fn update(&mut self, dt:f64) -> () {
        self.transform.update(dt);
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn render(&self, _camera:&Camera, canvas:&mut Canvas, pv:&Mat4x4) -> () {
        let uni = TextureUni{
            pvm: pv * &self.transform.model_conversion(),
            size: canvas.size(),
            texture: self.texture.clone()
        };
        self.mesh_renderer.render(canvas, &self.mesh, &uni);
    }
    fn is_terminated(&self) -> bool {
        self.terminated
    }
}