use crate::util::{Mat4x4, Vec4Model, Vec4Project };


pub fn default_vshader(
    pvm:&Mat4x4,
    point:&Vec4Model
) -> Vec4Project {
    Vec4Project::new(pvm * &point.0)
}