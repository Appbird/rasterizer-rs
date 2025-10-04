use crate::util::{ease, Point2, Vec4};

pub fn default_fshader(
    _point:Point2,
    color:Vec4
) -> Vec4 {
    return color;
}


pub fn fog_fshader(
    _point:Point2,
    depth:f64,
    color:&Vec4,
    background_color:&Vec4,
    far: f64, near: f64,
    fog_far:f64
) -> Vec4 {
    assert!(0. - 1e-6 <= depth || depth <= 1. + 1e-6);
    // ldepth
    let depth = (2.*far*near) / (far+near - (far-near)*depth);
    let depth = (depth - near)/(fog_far-near);
    let depth = f64::max(depth, 0.0);
    let depth = f64::min(depth / (fog_far), 1.0);
    let depth = depth*depth;
    return background_color*depth + color*(1. - depth);
}