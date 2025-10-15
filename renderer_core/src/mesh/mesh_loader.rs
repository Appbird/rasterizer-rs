use std::path::Path;

use gltf::Gltf;

use crate::{mesh::VertexArrayObject, shader::color_pipeline, util::{Throwable, Vec4}};

pub fn load_vao(path:&Path)
    -> Throwable<VertexArrayObject<color_pipeline::Attribute>> {
    let (model, buffers, _) = gltf::import(path)?;
    for mesh in model.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions =
                reader.read_positions().expect("The model data doesn't have position data.")
                .map(|point| Vec4::newpoint(point[0] as f64, point[1] as f64, point[2] as f64))
                .collect::<Vec<_>>();
            let colors =
                reader.read_colors(0).expect("The model data doesn't have color data.")
                .into_rgb_f32()
                .map(|color| Vec4::newvec(color[0] as f64, color[1] as f64, color[2] as f64))
                .collect::<Vec<_>>();
            
            let idcs = vec![];
            for (idx, vert_idx) in
                reader.read_indices()
                .expect("The model data doesn't have color data.")
                .into_u32()
                .enumerate() {
                
                if idx % 2 == 0 { vert_idx }
            }
        }
    }
    Ok(())
}