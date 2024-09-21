use std::ops;

use super::Point2;
#[derive(Clone)]
pub struct Vec4 {
    e: [f64; 4],
}

pub type Color = Vec4;

pub struct Vec4Screen(pub Vec4);
impl Vec4Screen {
    /** このVec4のx, y要素をとった新たなPoint2を作る */
    pub fn to_point2(&self) -> Point2 {
        Point2::new(self.0.x() as i32, self.0.y() as i32)
    }
}
pub struct Vec4Project(pub Vec4);
pub struct Vec4View(pub Vec4);
pub struct Vec4World(pub Vec4);
pub struct Vec4Model(pub Vec4);

impl Vec4 {
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn w(&self) -> f64 { self.e[3] }
    
    pub fn new(x: f64, y: f64, z: f64, w:f64) -> Self {
        Vec4 { e: [x, y, z, w] }
    }
    pub fn zero() -> Self {
        Vec4 { e: [0., 0., 0., 0.] }
    }
    fn bin<F>(lhs:&Self, rhs:&Self, op:F) -> Self
        where
        F:Fn(f64, f64) -> f64
    {
        Vec4::new(
            op(lhs.e[0], rhs.e[0]),
            op(lhs.e[1], rhs.e[1]),
            op(lhs.e[2], rhs.e[2]),
            op(lhs.e[3], rhs.e[3]),
        )
    }
    fn uni<F>(lhs:&Self, op:F) -> Self
        where
        F: Fn(f64) -> f64
    {
        Vec4::new(
            op(lhs.e[0]),
            op(lhs.e[1]),
            op(lhs.e[2]),
            op(lhs.e[3]),
        )
    }

    /** 要素ごとの積 */
    pub fn hadamard(self, rhs:&Self) -> Self { Vec4::bin(&self, rhs, |a, b|{a * b}) }
}

impl ops::Add for &Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4::bin(self, rhs, |a, b|{a + b})
    }
}
impl ops::Add for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Self) -> Self::Output { &self + &rhs }
}

impl ops::Sub for &Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec4::bin(&self, &rhs, |a, b|{a - b})
    }
}
impl ops::Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Self) -> Self::Output { &self - &rhs }
}
impl ops::Mul<f64> for &Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec4::uni(&self, |a|{a * rhs})
    }
}
impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f64) -> Self::Output { &self * rhs }
}
/** 交換則 */
impl ops::Mul<&Vec4> for f64 {
    type Output = Vec4;
    fn mul(self, rhs: &Vec4) -> Self::Output { rhs * self }
}

impl ops::Div<f64> for &Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f64) -> Self::Output {
        Vec4::uni(&self, |a|{a / rhs})
    }
}
impl ops::Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f64) -> Self::Output { &self / rhs }
}

