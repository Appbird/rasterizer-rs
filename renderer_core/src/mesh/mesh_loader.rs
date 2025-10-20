use std::path::Path;

use gltf::Primitive;
use image::{DynamicImage, ImageBuffer, ImageReader, Rgba32FImage};

use crate::{mesh::VertexArrayObject, shader::texture_pipeline, util::{Throwable, Vec4, Vec4Model}};

type VAOUV = VertexArrayObject<texture_pipeline::Attribute>;
type VAOUVImg = (VAOUV, Rgba32FImage);

pub fn load_vao(path:&Path)
    -> Throwable<Vec<VAOUVImg>> {
    let (model, buffers, images) = gltf::import(path)?;
    let mut vao_seq: Vec<VAOUVImg> = vec![];
    for mesh in model.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let points =
                reader.read_positions()
                .expect("The model data doesn't have position data.")
                .map(|point| Vec4::newpoint(point[0] as f64, point[2] as f64, point[1] as f64))
                .map(|p| Vec4Model(p))
                .collect::<Vec<_>>();
            let uvs =
                reader.read_tex_coords(0)
                .expect("The model data doesn't have color data.")
                .into_f32()
                .map(|point| Vec4::newvec(point[0] as f64, point[1] as f64, 0.))
                .collect::<Vec<_>>();
            
            assert_eq!(points.len(), uvs.len());
            let attribute =
            points.into_iter()
            .zip(uvs)
            .map(|(point, uv)| texture_pipeline::Attribute { point, uv })
            .collect::<Vec<_>>();
        
            let image= extract_image(&images, &primitive);
            let idx = 
                reader.read_indices()
                .expect("The model data doesn't have color data.")
                .into_u32()
                .map(|u| u as usize)
                .collect::<Vec<_>>()
                .chunks_exact(3)
                .map(|idx| [idx[0], idx[1], idx[2]])
                .collect::<Vec<_>>();
            vao_seq.push((VAOUV{ attribute, idx }, image));
        }
    }
    Ok(vao_seq)
}

fn extract_image(images: &Vec<gltf::image::Data>, primitive:&Primitive) -> Rgba32FImage {
    let material = primitive.material();
    let tex_image =
        material.pbr_metallic_roughness()
        .base_color_texture()
        .expect(&format!("There should be texture in primitive {}.", primitive.index()))
        .texture()
        .source()
        .source();
    let image =
        match tex_image {
            gltf::image::Source::View { view, .. } => {
                let buffer = &images[view.index()];
                println!("{:?}", buffer.format);
                let buffer =
                    image::ImageBuffer::from_raw(buffer.width, buffer.height, buffer.pixels.clone())
                    .expect(&format!("Failed to parse as image #{}", view.index()));
                DynamicImage::ImageRgba8(buffer).to_rgba32f()
            },
            gltf::image::Source::Uri { uri, .. } => {
                ImageReader::open(uri)
                .expect(&format!("Failed to open image in {}", uri))
                .decode()
                .expect(&format!("Failed to parse as image {}", uri))
                .to_rgba32f()
            },
        };
    image
}
