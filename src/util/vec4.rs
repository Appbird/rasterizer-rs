use std::ops;

use super::Point2;
#[derive(Clone, Debug)]
pub struct Vec4 {
    e: [f64; 4],
}

pub type Color = Vec4;
#[derive(Clone, Debug)]
pub struct Vec4Screen(pub Vec4);
impl Vec4Screen {
    /** このVec4のx, y要素をとった新たなPoint2を作る */
    pub fn to_point2(&self) -> Point2 {
        Point2::new((self.0.x()) as i32, (self.0.y()) as i32)
    }
}
#[derive(Clone, Debug)]
pub struct Vec4Project(Vec4);
impl Vec4Project {
	pub fn new(v:Vec4) -> Self {
		if v.w() < 1e-6 { Self(v*1e6) } else { Self(&v/v.w()) }
	}
	pub fn into_screen(&self, size:&Point2) -> Vec4Screen {
        let scale = size.y as f64 / 2.;
        let v = self.0.scaled_xy(&scale, &(-scale)) + size.to_vec4() / 2.;
		Vec4Screen(v)
    }
}

#[derive(Clone, Debug)]
pub struct Vec4View(pub Vec4);
#[derive(Clone, Debug)]
pub struct Vec4World(pub Vec4);
#[derive(Clone, Debug)]
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
    pub fn zero() -> Self {
        Vec4 { e: [0., 0., 0., 0.] }
    }
    pub fn newvec(x: f64, y: f64, z: f64) -> Self {
        Vec4 { e: [x, y, z, 0.] }
    }
	pub fn newpoint(x: f64, y: f64, z: f64) -> Self {
        Vec4 { e: [x, y, z, 1.] }
    }
	pub fn newpixel(x: i32, y: i32) -> Self {
        Vec4 { e: [x as f64 + 0.5, y as f64 + 0.5, 0., 1.] }
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

	pub fn cross2d(&self, rhs:&Self) -> f64 {
		self.x()*rhs.y() - self.y()*rhs.x()
	}
	pub fn cross3d(&self, rhs:&Self) -> Vec4 {
		let (x1, y1, z1) = (self.x(), self.y(), self.z());
		let (x2, y2, z2) = (rhs.x(), rhs.y(), rhs.z());
		Vec4::new(
			y1*z2 - z1*y2,
			z1*x2 - x1*z2,
			x1*y2 - y1*x2,
			0.
		)
	}
	pub fn normalized3d(&self) -> Vec4 {
		let (x, y, z) = (self.x(), self.y(), self.z());
		let n = f64::sqrt(x*x + y*y + z*z);
		Vec4::new(x/n, y/n, z/n, self.w())
	}
    pub fn norm3d(&self) -> f64 {
        let (x, y, z) = (self.x(), self.y(), self.z());
        return f64::sqrt(x*x + y*y + z*z);
    }
    pub fn to_point2(&self) -> Point2 {
		Point2::new(self.x() as i32, self.y() as i32)
	}

	pub fn scaled_xy(&self, x: &f64, y: &f64) -> Self {
        Vec4 { e: [self.x() * x, self.y() * y, self.z(), self.w()] }
    }
}

impl ops::Add for &Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4::construct(|i| self.e[i] + rhs.e[i])
    }
}
impl ops::AddAssign<&Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: &Self) {
        for i in 0..4 { self.e[i] += rhs.e[i]; }
    }
}
impl ops::AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 { self.e[i] += rhs.e[i]; }
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

// 単項マイナス演算子の実装
impl ops::Neg for &Vec4 {
	type Output = Vec4;
	fn neg(self) -> Self::Output {
		Vec4::construct(|i| -self.e[i])
	}
}
impl ops::Neg for Vec4 {
	type Output = Vec4;
	fn neg(self) -> Self::Output {
		-&self
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

