use std::ops;

use super::Point2;
#[derive(Clone, Debug)]
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
    pub fn i(&self, i:usize) -> f64 { self.e[i] }
    
    pub fn new(x: f64, y: f64, z: f64, w:f64) -> Self {
        Vec4 { e: [x, y, z, w] }
    }
    pub fn new3d(x: f64, y: f64, z: f64) -> Self {
        Vec4 { e: [x, y, z, 0.] }
    }
    pub fn from_array(v: [f64; 4]) -> Self {
        Vec4 { e: v }
    }
    /** `i`番目の要素が`element(i)`であるようなベクトルを構成する。
     * ただし、`i`について0は`x`, 1は`y`, 2は`z`, 3は`w`座標をそれぞれ示す。
    */
    pub fn construct<F>(element:F) -> Self
    where 
        F:Fn(usize) -> f64
    {
        Vec4::new(
            element(0),
            element(1),
            element(2),
            element(3),
        )
    }
    fn fold(self) -> f64 {
        self.e.iter().fold(0., |item, r| item + r)
    }

    /** 要素ごとの積 */
    pub fn hadamard(&self, rhs:&Self) -> Self { Vec4::construct(|i| self.e[i] * rhs.e[i]) }
    /** 内積 */
    pub fn dot(&self, rhs:&Self) -> f64 { self.hadamard(rhs).fold() }
    
}

impl ops::Add for &Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4::construct(|i| self.e[i] + rhs.e[i])
    }
}
impl ops::Add for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Self) -> Self::Output { &self + &rhs }
}

impl ops::Sub for &Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec4::construct(|i| self.e[i] - rhs.e[i])
    }
}
impl ops::Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Self) -> Self::Output { &self - &rhs }
}
impl ops::Mul<f64> for &Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec4::construct(|i| self.e[i] * rhs)
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
        Vec4::construct(|i| self.e[i] / rhs)
    }
}
impl ops::Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f64) -> Self::Output { &self / rhs }
}

