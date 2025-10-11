use std::ops::{Add, Mul};

use image::Rgba32FImage;

use crate::{shader::pipeline::{Interpolable, RenderingPipeline}, util::{Mat4x4, Point2, Vec4}};


pub struct TexturePipeline;
struct Uniform {
    pvm:Mat4x4,
    texture:Rgba32FImage
}
struct Attribute {
    point:Vec4,
    uv:Vec4
}
macro_rules! impl_varying_fields {
    ($ty:ident { $($field:ident),+ $(,)? }) => {
        
        impl Interpolable for $ty {
            fn interpolate(p:[&Self;3], w:[f64; 3]) -> Self {
                Self {
                    $(
                        $field: &p[0].$field * w[0] + &p[1].$field * w[1] + &p[2].$field * w[2],
                    ) +
                }
            }
        }

    }
}

struct Varying {
    depth:Vec4,
    uv:Vec4
}
impl_varying_fields!(Varying{ depth, uv });

impl RenderingPipeline<Uniform, Attribute, Varying> for TexturePipeline{
    fn vertex(uni:&Uniform, vert:&Attribute, out:&mut Varying) -> Varying {
        let v = &uni.pvm * &vert.point;
        out = Self{ depth, uv };
    }

    fn fragment(p:&Point2, uni:&Uniform, vary:&Varying) -> crate::util::Vec4 {
        todo!()
    }
}