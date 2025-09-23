use crate::util::{Mat4x4, Vec4, Vec4Project};


pub fn default_vshader(
    pvm:&Mat4x4,
    point:&Vec4
) -> Vec4Project {
    Vec4Project::new(pvm * point)
}