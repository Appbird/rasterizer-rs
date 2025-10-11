use crate::util::{Mat4x4, Vec4};
use std::f64::consts::PI;

pub struct Camera {
    pub near:f64,
    pub far:f64,
    pub width:f64,
    pub height:f64,
	pub position:Vec4,
	pub up:Vec4,
	pub look:Vec4,
}

impl Camera {
    pub fn new(aspect: f64) -> Camera {
        let near = 0.1;
        let width = 2. * near * f64::tan(PI /6.);
        Camera{
            near: 0.1,
            far: 100.,
            width,
            height: width * aspect,
			position: Vec4::newpoint(0., 0., 0.),
			look: Vec4::newvec(0., 0., -1.),
			up: Vec4::newvec(0., 1., 0.),
        }
    }
    pub fn perspective_conversion(&self) -> Mat4x4 {
        let n = self.near;
        let f = self.far;
        let w = self.width;
        let h = self.height;
		
        Mat4x4::from_array([
            [ n / w,    0.,     0.,     0.  ],
            [ 0.,       n / h,  0.,     0.  ],
            [ 0.,       0.,     -(f+n)/(f-n), -2.*(f*n)/(f-n)],
            [ 0.,       0.,     -1.,     0.  ],
        ])
    }
	pub fn view_conversion(&self) -> Mat4x4 {
		let ez = -&self.look;
		let ex = &self.up.cross3d(&ez).normalized3d();
		let ey = ez.cross3d(&ex);
        Mat4x4::transposed_basis(&ex, &ey, &ez) * Mat4x4::translate(&-(&self.position))
    }
}
