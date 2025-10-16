use std::path::Path;
use crate::{mesh::VertexArrayObject, shader::color_pipeline, util::{Throwable, Vec4, Vec4Model}};

type VAOColor = VertexArrayObject<color_pipeline::Attribute>;

pub fn load_vao(path:&Path)
    -> Throwable<Vec<VAOColor>> {
    let (model, buffers, _) = gltf::import(path)?;
    let mut vao_seq: Vec<VAOColor> = vec![];
    for mesh in model.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let points =
                reader.read_positions().expect("The model data doesn't have position data.")
                .map(|point| Vec4::newpoint(point[0] as f64, point[1] as f64, point[2] as f64))
                .map(|p| Vec4Model(p))
                .collect::<Vec<_>>();
            let colors =
                reader.read_colors(0).expect("The model data doesn't have color data.")
                .into_rgb_f32()
                .map(|color| Vec4::newvec(color[0] as f64, color[1] as f64, color[2] as f64))
                .collect::<Vec<_>>();
            
            assert_eq!(points.len(), colors.len());
            let attribute =
                points.into_iter()
                .zip(colors)
                .map(|(point, color)| color_pipeline::Attribute{ point, color })
                .collect::<Vec<_>>();

            let idx = 
                reader.read_indices()
                .expect("The model data doesn't have color data.")
                .into_u32()
                .map(|u| u as usize)
                .collect::<Vec<_>>()
                .chunks_exact(3)
                .map(|idx| [idx[0], idx[1], idx[2]])
                .collect::<Vec<_>>();
            vao_seq.push(VAOColor { attribute, idx });
        }
    }
    Ok(vao_seq)
}