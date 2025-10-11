
use crate::util::{Point2, Vec4};

pub trait Interpolable: Sized {
    fn interpolate(
        p:[&Self; 3],
        w:[f64; 3]
    ) -> Self;
}

pub trait RenderingPipeline<
    Uniform,
    Attribute,
    Varying,
> where Varying: Interpolable
{
    fn vertex(uni:&Uniform, vert:&Attribute) -> Varying;
    fn fragment(p:&Point2, uni:&Uniform, vary:&Varying) -> Vec4;
}
