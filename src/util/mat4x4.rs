use std::ops::Mul;

use super::Vec4;
#[derive(Debug)]
pub struct Mat4x4 {
    e: [[f64; 4]; 4],
}
impl Mat4x4 {
    pub fn construct<F>(element:F) -> Mat4x4
    where  
        F: Fn(usize, usize) -> f64
    {
        let mut e: [[f64; 4]; 4] = [[0.; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                e[r][c] = element(r, c);
            }
        }
        Mat4x4{e}
    }
    /** Rodriguesの回転行列を使って`axis`軸周りに角度`theta`だけ点を回転させる行列を作る */
    pub fn rotation(axis:Vec4, theta: f64) -> Mat4x4 {
        let n = [axis.x(), axis.y(), axis.z()];
        let (cos, sin) = (f64::cos(theta), f64::sin(theta));
        let lcos = 1. - cos;
        Self::construct(
            |r, c|
            if (r, c) == (3, 3) { 1. }
            else if r == 3 || c == 3 { 0. }
            else if r == c {
                cos + n[r]*n[c]*lcos
            } else {
                let sign = if
                    (r, c) == (2, 1) ||
                    (r, c) == (1, 0) ||
                    (r, c) == (0, 2)
                { 1. } else { -1. };
                n[r]*n[c]*lcos + sign * n[3-r-c]*sin
            }
        )
    }
    /** s倍するアフィン変換行列 */
    pub fn scale(s:Vec4) -> Mat4x4 {
        Self::construct(
            |r, c|
            if r != c { 0. }
            else if r != 4 { s.i(r) }
            else { 1. }
        )
    }
    /** v方向に移動させるアフィン変換行列を作る */
    pub fn translate(v:Vec4) -> Mat4x4 {
        Self::construct(
            |r, c|
            if r == c { 1. }
            else if c == 3 { v.i(r) }
            else { 0. }
        )
    }
}

impl Mat4x4 {
    /** 行列の`r`行目の行ベクトルを取り出す。 */
    fn row(&self, r:usize) -> Vec4 {
        Vec4::construct(|i| self.e[r][i])
    }
    /** 行列の`c`列目の列ベクトルを取り出す。 */
    fn col(&self, c:usize) -> Vec4 {
        Vec4::construct(|i| self.e[i][c])
    }
}

impl Mul<&Vec4> for Mat4x4 {
    type Output = Vec4;
    fn mul(self, rhs: &Vec4) -> Self::Output {
        Vec4::construct(|r| self.row(r).dot(&rhs))
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        Mat4x4::construct(|r, c| self.row(r).dot(&rhs.col(c)))
    }
}
