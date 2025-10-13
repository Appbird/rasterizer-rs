
use crate::{shader::pipeline::RenderingPipeline, util::{Mat4x4, Point2, Vec4, Vec4Model, Vec4Project, Vec4Screen}};
use crate::shader::pipeline::Interpolable;
use interpolate_derive::Interpolate;

#[derive(Clone)]
pub struct ColorPipeline;
pub struct Uniform {
    pub pvm:Mat4x4,
    pub size:Point2,
    pub background_color:Vec4,
    pub far:f64,
    pub near:f64,
    pub fog_far:f64,
}

#[derive(Clone)]
pub struct Attribute {
    pub point:Vec4Model,
    pub color:Vec4,
}

#[derive(Interpolate)]
pub struct Varying {
    color: Vec4,
    inv_z:f64,
}


impl RenderingPipeline for ColorPipeline{
    type Uniform = Uniform;
    type Attribute = Attribute;
    type Varying = Varying;
    fn vertex(uni:&Uniform, vert:&Self::Attribute) -> (Varying, Vec4Screen) {
        let v = Vec4Project::new(&uni.pvm * &vert.point.0);
        (
            Varying{
                color: vert.color.clone(),
                inv_z: 1./ v.w()
            }, 
            v.into_screen(&uni.size)
        )
    }

    fn fragment(_p:&Point2, uni:&Uniform, vary:&Varying) -> Vec4 {
        let far = uni.far;
        let near = uni.near;
        let fog_far = uni.fog_far;
        let z_value = 1./vary.inv_z;
        println!("{}", z_value);

        let depth = (2.*far*near) / (far+near - (far-near)*z_value);
        let depth = (depth - near)/(fog_far - near);
        let depth = f64::clamp(depth, 0.0, 1.0);
        let fog = depth*depth;
        return &uni.background_color*fog + &vary.color*(1. - fog);
    }
}