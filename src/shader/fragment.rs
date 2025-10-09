use crate::util::{Point2, Vec4};

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
    let depth = (2.*far*near) / (far+near - (far-near)*depth);
    let depth = (depth - near)/(fog_far - near);
    let depth = f64::clamp(depth, 0.0, 1.0);
    let fog = depth*depth;
    return background_color*fog + color*(1. - fog);
}