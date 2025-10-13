use std::{f64::consts::PI, time::Instant};

use crate::{actor::Actor, canvas::Canvas, component::{MeshRenderer, Transform}, shader::{color_pipeline::ColorPipeline, pipeline::RenderingPipeline}, util::{ease, Mat4x4, Vec4, Vec4Model, VertexArrayObject}, world::camera::Camera};

type ColorUni = <ColorPipeline as RenderingPipeline>::Uniform;
type ColorAttr = <ColorPipeline as RenderingPipeline>::Attribute;

pub fn tetrahedron_mesh(vert_color:[Vec4; 4]) -> VertexArrayObject<ColorAttr> {
    let vectors = [
        Vec4::newpoint(0., 1., 0.),
        Vec4::newpoint(f64::cos(0.), -1./3., f64::sin(0.)),
        Vec4::newpoint(f64::cos(2.*PI/3.), -1./3., f64::sin(2.*PI/3.)),
        Vec4::newpoint(f64::cos(4.*PI/3.), -1./3., f64::sin(4.*PI/3.)),
    ];
    let indecies: Vec<[usize; 3]> = vec![
        [1, 2, 3],
        [0, 2, 3],
        [0, 1, 3],
        [0, 1, 2],
    ];
    let mut vertices: Vec<Vec4Model> = vec![];
    let mut colors:Vec<Vec4> = vec![];
    let mut v_idx_list: Vec<[usize; 3]> = vec![];
    for (idx, index_array) in indecies.into_iter().enumerate() {
        let tri_vertices = vec![
            &vectors[index_array[0]],
            &vectors[index_array[1]],
            &vectors[index_array[2]]
        ];
        let g = (&(tri_vertices[0] + tri_vertices[1]) + tri_vertices[2])  / 3.;
        for i in 0..3 {
            let v = &((tri_vertices[i] - &g) * 0.95) + &g;
            vertices.push(Vec4Model(v));
            colors.push(vert_color[index_array[i]].clone());
        }
        v_idx_list.push([0 + idx * 3, 1 + idx*3, 2 + idx*3]);
    };
    VertexArrayObject::<_>{
        attribute: vertices.into_iter().zip(colors)
            .map(|(point, color)| { ColorAttr{ point, color } })
            .collect(),
        idx: v_idx_list
    }
}


#[derive(Clone)]
pub struct TetrahedronActor {
    pub mesh:VertexArrayObject<ColorAttr>,
    pub transform:Transform,
    pub mesh_renderer: MeshRenderer<ColorPipeline>,
    created_at: Instant,
    terminated:bool,
}

impl TetrahedronActor {
    pub fn new(
        mesh:VertexArrayObject<ColorAttr>
    ) -> Self {
        Self {
            mesh,
            transform: Transform::new(),
            mesh_renderer: MeshRenderer::new(ColorPipeline{}),
            created_at: Instant::now(),
            terminated: false
        }
    }
}

impl Actor for TetrahedronActor {
    fn update(&mut self, dt:f64) -> () {
        self.transform.update(dt);

        let t = self.created_at.elapsed().as_secs_f64();
        let scale_t = f64::min(t / 0.5, 1.);
        self.transform.scale = Vec4::newvec(1., 1., 1.) * 1.1 * ease::out_back(scale_t);
        if t > 3.0 { self.terminated = true; }
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