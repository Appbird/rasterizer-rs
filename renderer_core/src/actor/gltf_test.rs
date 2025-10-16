
use crate::{actor::Actor, canvas::Canvas, component::{MeshRenderer, Transform}, mesh::VertexArrayObject, shader::{color_pipeline::ColorPipeline, pipeline::RenderingPipeline}, util::Mat4x4, world::camera::Camera};

type ColorUni = <ColorPipeline as RenderingPipeline>::Uniform;
type ColorAttr = <ColorPipeline as RenderingPipeline>::Attribute;


#[derive(Clone)]
pub struct GltfTestActor {
    pub mesh:VertexArrayObject<ColorAttr>,
    pub transform:Transform,
    pub mesh_renderer: MeshRenderer<ColorPipeline>,
    terminated:bool,
}

impl GltfTestActor {
    pub fn new(
        mesh:VertexArrayObject<ColorAttr>
    ) -> Self {
        let mut mesh_renderer = MeshRenderer::new(ColorPipeline{});
        mesh_renderer.culling(true);
        Self {
            mesh,
            transform: Transform::new(),
            mesh_renderer,
            terminated: false
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
    fn render(&self, camera:&Camera, canvas:&mut Canvas, pv:&Mat4x4) -> () {
        let uni = ColorUni{
            pvm: pv * &self.transform.model_conversion(),
            size: canvas.size(),
            background_color: canvas.background_color.clone(),
            near: camera.near,
            fog_far: 40.,
        };
        self.mesh_renderer.render(canvas, &self.mesh, &uni);
    }
    fn is_terminated(&self) -> bool {
        self.terminated
    }
}