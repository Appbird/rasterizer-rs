
use std::rc::Rc;

use image::Rgba32FImage;

use crate::{shader::pipeline::RenderingPipeline, util::{Mat4x4, Point2, Vec4, Vec4Model, Vec4Project, Vec4Screen}};
use crate::shader::pipeline::Interpolable;
use interpolate_derive::Interpolate;

#[derive(Clone)]
pub struct TexturePipeline;
pub struct Uniform {
    pub pvm:Mat4x4,
    pub size:Point2,
    pub texture:Rc<Rgba32FImage>
}
#[derive(Clone)]
pub struct Attribute {
    pub point:Vec4Model,
    pub uv:Vec4
}
#[derive(Interpolate)]
pub struct Varying {
    uv_inv_z:Vec4,
    inv_z:f64
}
impl RenderingPipeline for TexturePipeline{
    type Uniform = Uniform;
    type Attribute = Attribute;
    type Varying = Varying;
    fn vertex(uni:&Uniform, vert:&Self::Attribute) -> (Varying, Vec4Screen) {
        let v = Vec4Project::new(&uni.pvm * &vert.point.0);
        (
            Varying{
                uv_inv_z: vert.uv.clone() / v.w(),
                inv_z: 1. / v.w()
            }, 
            v.into_screen(&uni.size)
        )
    }

    fn fragment(_p:&Point2, uni:&Uniform, vary:&Varying) -> Vec4 {
        let (w, h) = uni.texture.dimensions();
        let z = 1. / vary.inv_z;
        let uv = &vary.uv_inv_z * z;
        let (u, v) = (uv.x() * w as f64, uv.y() * h as f64);
        let (u, v) = (u as u32, v as u32);
        let pixel = uni.texture.get_pixel_checked(u, v);
        if let Some(c) = pixel {
            Vec4::new(c[0] as f64, c[1] as f64, c[2] as f64, c[3] as f64)
        } else {
            Vec4::new(0., 0., 0., 0.)
        }
    }
}