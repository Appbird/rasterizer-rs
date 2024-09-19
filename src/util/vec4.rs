use std::ops;
pub struct Vec4 {
    e: [f64; 4],
}

pub type Color = Vec4;

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
    fn bin<F>(lhs:Self, rhs:Self, op:F) -> Self
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
    fn uni<F>(lhs:Self, op:F) -> Self
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
}

impl ops::Add for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4::bin(self, rhs, |a, b|{a + b})
    }
}
impl ops::Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec4::bin(self, rhs, |a, b|{a - b})
    }
}
impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec4::uni(self, |a|{a * rhs})
    }
}
impl ops::Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f64) -> Self::Output {
        Vec4::uni(self, |a|{a / rhs})
    }
}
impl ops::Mul<Vec4> for f64 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output { rhs * self }
}