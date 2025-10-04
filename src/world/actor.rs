use std::{f64::consts::PI, time::Instant};

use crate::util::{ease, Mat4x4, Vec4};

#[derive(Clone)]
pub struct Actor {
    pub polygons:Vec<Polygon>,
    pub position:Vec4,
    pub velocity:Vec4,
    pub acc:Vec4,
    pub theta:Vec4,
    pub omega:Vec4,
    pub angacc:Vec4,
    pub scale: Vec4,
    created_at: Instant,
    terminated:bool,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: [Vec4; 3],
    pub color: [Vec4; 3],
}

impl Actor {
    pub fn new(
        polygon:Vec<Polygon>
    ) -> Actor {
        let zerovec =  Vec4::newvec(0., 0., 0.);
        Actor {
            polygons: polygon,
            position: Vec4::newpoint(0., 0., 0.),
            velocity: zerovec.clone(),
            acc: zerovec.clone(),
            theta: Vec4::newvec(2.*PI, 0., 0.),
            omega: zerovec.clone(),
            angacc: zerovec.clone(),
            scale: Vec4::newvec(1., 1., 1.),
            created_at: Instant::now(),
            terminated: false
        }
    }
    pub fn update(&mut self, dt:f64) -> () {
        self.position += &self.velocity * dt;
        self.velocity += &self.acc * dt;
        self.theta += &self.omega * dt;
        self.omega += &self.angacc * dt;
        
        let t = self.created_at.elapsed().as_secs_f64();
        
        let scale_t = f64::min(t / 0.5, 1.);
        self.scale = Vec4::newvec(1., 1., 1.) * 1.1 * ease::out_back(scale_t);

        if t > 3.0 { self.terminate(); }
    }
    pub fn model_conversion(&self) -> Mat4x4 {
        let t = self.theta.norm3d();
        Mat4x4::translate(&self.position)
        * Mat4x4::rotation(&(&self.theta / t), t)
        * Mat4x4::scale(&self.scale)
    }
    pub fn terminate(&mut self) {
        self.terminated = true;
    }
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
}