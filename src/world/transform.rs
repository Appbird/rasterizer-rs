use std::f64::consts::PI;

use crate::util::{Mat4x4, Vec4};


#[derive(Clone)]
pub struct Transform {
    pub position:Vec4,
    pub velocity:Vec4,
    pub acc:Vec4,

    pub theta:Vec4,
    pub omega:Vec4,
    pub angacc:Vec4,
    
    pub scale: Vec4,
}

impl Transform {
    pub fn new() -> Self {
        let zerovec =  Vec4::newvec(0., 0., 0.);
        Transform {
            position: Vec4::newpoint(0., 0., 0.),
            velocity: zerovec.clone(),
            acc: zerovec.clone(),
            theta: Vec4::newvec(2.*PI, 0., 0.),
            omega: zerovec.clone(),
            angacc: zerovec.clone(),
            scale: Vec4::newvec(1., 1., 1.)
        }
    }
    pub fn update(&mut self, dt:f64) {
        self.position += &self.velocity * dt;
        self.velocity += &self.acc * dt;
        self.theta += &self.omega * dt;
        self.omega += &self.angacc * dt;
    }
    pub fn model_conversion(&self) -> Mat4x4 {
        let t = self.theta.norm3d();
        Mat4x4::translate(&self.position)
        * Mat4x4::rotation(&(&self.theta / t), t)
        * Mat4x4::scale(&self.scale)
    }
}