use super::Vec4;

#[derive(Clone, Debug, Copy)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }
    pub fn flipped(&self) -> Self {
        Point2::new(self.y, self.x)
    }
    pub fn to_vec4(self) -> Vec4 {
        Vec4::new(self.x as f64, self.y as f64, 0., 0.)
    }
}