use crate::{
    canvas::Canvas,
    util::{Color, Point2, Vec4Project, Vec4Screen}
};

pub struct Camera {
}

impl Camera {
    pub fn new() -> Camera { Camera{} }
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
