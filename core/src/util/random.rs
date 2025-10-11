use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;
use crate::util::Vec4;

impl Vec4 {
    pub fn cone_zplus(rng:&mut ThreadRng) -> Vec4 {
        let theta = rng.random_range(0.0..2.*PI); 
        Vec4::newvec(f64::cos(theta), f64::sin(theta), 1.).normalized3d()
    }
    pub fn choice_in_sphere(rng:&mut ThreadRng) -> Vec4 {
        
        let mut x = 1.;
        let mut y = 1.;
        let mut z = 1.;
        while x*x + y*y + z*z > 1. {
            x = rng.random_range(0.0..1.) + 1e-6;
            y = rng.random_range(0.0..1.) + 1e-6;
            z = rng.random_range(0.0..1.) + 1e-6;
        }
        Vec4::newvec(x, y, z).normalized3d()
    }
}