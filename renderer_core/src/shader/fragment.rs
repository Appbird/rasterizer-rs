use image::Rgba32FImage;

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

pub fn texture_fshader(
    _point:Point2,
    uv:&Vec4,
    texture:&Rgba32FImage
) -> Vec4 {
    let (w, h) = texture.dimensions();
    let (u, v) = (uv.x() * w as f64, uv.y() * h as f64);
    let (u, v) = (u as u32, v as u32);
    let pixel = texture.get_pixel_checked(u, v);
    if let Some(c) = pixel {
        Vec4::new(c[0] as f64, c[1] as f64, c[2] as f64, c[3] as f64)
    } else {
        Vec4::new(0., 0., 0., 0.)
    }
}