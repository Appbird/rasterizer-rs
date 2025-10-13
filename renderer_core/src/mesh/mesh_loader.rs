use fbxcel_dom::any::AnyDocument;
use std::{fs::File, io::Result, path::Path};
use crate::{mesh::VertexArrayObject, shader::texture_pipeline};


fn load_vao(path:&Path) -> Result<VertexArrayObject<texture_pipeline::Attribute>> {
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    match AnyDocument::from_seekable_reader(reader).expect("Failed to load document") {
        AnyDocument::V7400(fbx_ver, doc) => {
            let tree = doc.tree();
            
        }
        _ => panic!("Got FBX document of unsupported version.")
    };
    Ok(())

}