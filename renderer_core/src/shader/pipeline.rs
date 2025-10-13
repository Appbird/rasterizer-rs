
use crate::util::{Point2, Vec4, Vec4Screen};

pub trait Interpolable: Sized {
    fn interpolate(
        p:&[&Self; 3],
        w:&[f64; 3]
    ) -> Self;
}

pub trait RenderingPipeline
{
    type Uniform;
    type Attribute;
    type Varying: Interpolable;
    fn vertex(uni:&Self::Uniform, vert:&Self::Attribute) -> (Self::Varying, Vec4Screen);
    fn fragment(p:&Point2, uni:&Self::Uniform, vary:&Self::Varying) -> Vec4;
}
