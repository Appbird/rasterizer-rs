use crate::util::{Point2, Vec4};

pub fn default_fshader(
    _point:Point2,
    color:Vec4
) -> Vec4 {
    return color;
}