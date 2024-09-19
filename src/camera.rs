use crate::{
    canvas::Canvas,
    util::{Color, Point2, Vec4}
};

struct Camera {
    canvas:Canvas
}

impl Camera {
    fn new(canvas:Canvas) -> Camera{
        Camera{canvas}
    }
    fn draw_line(&mut self, p1:&Vec4, p2:&Vec4, color:&Color) {
        let p1 = Point2::new(p1.x() as i32, p1.y() as i32);
        let p2 = Point2::new(p2.x() as i32, p2.y() as i32);
        self.canvas.draw_line(&p1, &p2, color);
    }
}
