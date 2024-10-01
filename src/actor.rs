use crate::util::{Mat4x4, Vec4};

pub struct Actor {
    pub vertices:[Vec4; 3],
    pub position:Vec4,
    pub axis:Vec4,
    pub theta:f64,
    pub color: Vec4
}

impl Actor {
    pub fn model_conversion(&self) -> Mat4x4 {
        Mat4x4::translate(&self.position)
        * Mat4x4::rotation(&self.axis, self.theta)
    }
}