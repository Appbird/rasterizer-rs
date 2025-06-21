use crate::{
    actor::Actor, canvas::Canvas, util::{Mat4x4, Vec4, Vec4Project}
};
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
        let width = 2. * near * f64::tan(PI /4.);
        Camera{
            near: 0.1,
            far: 100.,
            width,
            height: width * aspect,
			position: Vec4::newpoint(0., 0., 6.),
			look: Vec4::newvec(0., 0., -1.),
			up: Vec4::newvec(0., 1., 0.),
        }
    }
    pub fn snapshot(&mut self, canvas:&mut Canvas, actors:&Vec<Actor>) {
        let perspective = self.perspective_conversion();
        let view = self.view_conversion();
        let pv = perspective*view;
        for actor in actors {
            let model = actor.model_conversion();
            let pvm = &pv * &model;
            let projected = [
                Vec4Project::new(&pvm * &actor.vertices[0]).into_screen(&canvas.size()),
                Vec4Project::new(&pvm * &actor.vertices[1]).into_screen(&canvas.size()),
                Vec4Project::new(&pvm * &actor.vertices[2]).into_screen(&canvas.size()),
            ];
            canvas.draw_triangle(projected, &actor.color);
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
            [ 0.,       0.,     (f+n)/(f-n), -2.*(f*n)/(f-n)],
            [ 0.,       0.,     1.,     0.  ],
        ])
    }
	pub fn view_conversion(&self) -> Mat4x4 {
		let ez = &self.look;
		let ex = ez.cross3d(&self.up).normalized3d();
		let ey = ez.cross3d(&ex).normalized3d();
        Mat4x4::transposed_basis(&ex, &ey, &ez) * Mat4x4::translate(&-(&self.position))
    }
}
