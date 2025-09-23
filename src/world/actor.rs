use crate::util::{Mat4x4, Vec4};

pub struct Actor {
    pub polygons:Vec<Polygon>,
    pub position:Vec4,
    pub velocity:Vec4,
    pub acc:Vec4,
    pub theta:Vec4,
    pub omega:Vec4,
    pub angacc:Vec4,
    pub scale: Vec4
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: [Vec4; 3],
    pub color: [Vec4; 3],
}

impl Actor {
    pub fn update(&mut self, dt:f64) -> () {
        self.position += &self.velocity * dt;
        self.velocity += &self.acc * dt;
        self.theta += &self.omega * dt;
        self.omega += &self.angacc * dt;
    }
    pub fn model_conversion(&self) -> Mat4x4 {
        let t = self.theta.norm3d();
        Mat4x4::translate(&self.position)
        * Mat4x4::rotation(&(&self.theta / t), t)
    }
}