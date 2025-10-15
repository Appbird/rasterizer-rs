use std::path::Path;

use gltf::Gltf;

use crate::{mesh::VertexArrayObject, shader::texture_pipeline, util::Throwable};

fn load_vao(path:&Path)
    -> Throwable<()> {
    //-> Throwable<VertexArrayObject<texture_pipeline::Attribute>> {
    let model = Gltf::open(path)?;
    for mesh in model.meshes() {
        println!("Mesh #{}", mesh.index());
        for primitive in mesh.primitives() {
            println!("- primitive #{}", primitive.index());
            for (semantic, _) in primitive.attributes() {
                println!("-- {semantic:?}");
            }
        }
    }
    Ok(())
}