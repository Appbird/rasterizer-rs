use std::ops;

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
	pub fn at(&self, idx:usize) -> &i32 {
		if idx == 0 { &self.x } else { &self.y }
	}
	pub fn dot(&self, rhs:&Self) -> i32 {
		&self.x * rhs.x  + &self.y * rhs.y
	}
	pub fn cross(&self, rhs:&Self) -> i32 {
		self.x * rhs.y - self.y * rhs.x
	}
	
}


impl ops::Add for &Point2 {
    type Output = Point2;
    fn add(self, rhs: Self) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y )
    }
}
impl ops::Add for Point2 {
    type Output = Point2;
    fn add(self, rhs: Self) -> Self::Output { &self + &rhs }
}

impl ops::Sub for &Point2 {
    type Output = Point2;
    fn sub(self, rhs: Self) -> Self::Output {
        Point2::new(self.x - rhs.x, self.y - rhs.y )
    }
}
impl ops::Sub for Point2 {
    type Output = Point2;
    fn sub(self, rhs: Self) -> Self::Output { &self - &rhs }
}