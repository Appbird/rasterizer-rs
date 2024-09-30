use crate::util::{Mat4x4, Vec4};

pub struct Actor {
    pub vertices:[Vec4; 3],
    position:Vec4,
    axis:Vec4,
    theta:f64,
}

impl Actor {
    fn model_conversion(&self) -> Mat4x4 {
        Mat4x4::translate(&self.position)
        * Mat4x4::rotation(&self.axis, self.theta)
    }
}