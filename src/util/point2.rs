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
}