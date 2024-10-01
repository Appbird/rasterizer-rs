use crate::{
    actor::Actor, canvas::Canvas, util::{Color, Mat4x4, Point2, Vec4, Vec4Project, Vec4Screen}
};
use std::f64::consts::PI;

pub struct Camera {
    near:f64,
    far:f64,
    width:f64,
    height:f64
}

impl Camera {
    pub fn new(aspect: f64) -> Camera {
        let near = 0.1;
        let width = 2. * near * f64::tan(PI /4.);
        Camera{
            near,
            far: 1000.,
            width,
            height: width * aspect,
        }
    }
    pub fn snapshot(&mut self, canvas:&mut Canvas, actors:&Vec<Actor>) {
        let perspective = self.view_conversion();
        let view = Mat4x4::identity();
        let pv = perspective*view;

        for actor in actors {
            let model = actor.model_conversion();
            let pvm = &pv * &model;
            let projected:[Vec4Project; 3] = [
                Vec4Project(&pvm * &actor.vertices[0]),
                Vec4Project(&pvm * &actor.vertices[1]),
                Vec4Project(&pvm * &actor.vertices[2]),
            ];
            self.draw_triangle(canvas, &projected[0], &projected[1], &projected[2], &actor.color);
        }
    }
    pub fn view_conversion(&self) -> Mat4x4 {
        let n = self.near;
        let f = self.far;
        let w = self.width;
        let h = self.height;
        Mat4x4::from_array([
            [ n / w,    0.,     0.,     0.  ],
            [ 0.,       n / h,  0.,     0.  ],
            [ 0.,       0.,     (f+n)/(f-n), -2.*(f*n)/(f - n)],
            [ 0.,       0.,     1.,     0.  ],
        ])
    }
    pub fn draw_point(&mut self, canvas:&mut Canvas, p1:&Vec4Project, color:&Color) {
        let p1 = self.transform_into_screen(canvas.size(), &p1).to_point2();
        canvas.draw_point(&p1, color);
    }
    pub fn draw_line(&mut self, canvas:&mut Canvas, p1:&Vec4Project, p2:&Vec4Project, color:&Color) {
        let p1 = self.transform_into_screen(canvas.size(), &p1).to_point2();
        let p2 = self.transform_into_screen(canvas.size(), &p2).to_point2();
        canvas.draw_line(&p1, &p2, color);
    }
    pub fn draw_triangle(&mut self, canvas:&mut Canvas, p1:&Vec4Project, p2:&Vec4Project, p3:&Vec4Project, color:&Color) {
        let p1 = self.transform_into_screen(canvas.size(), &p1).to_point2();
        let p2 = self.transform_into_screen(canvas.size(), &p2).to_point2();
        let p3 = self.transform_into_screen(canvas.size(), &p3).to_point2();
        canvas.draw_triangle(&[&p1, &p2, &p3], color);
    }
    pub fn transform_into_screen(&self, size:Point2, p: &Vec4Project) -> Vec4Screen {
        let scale = size.y as f64 / 2.;
        Vec4Screen(scale * &p.0 + size.to_vec4() / 2.)
    }
}
