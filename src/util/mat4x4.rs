use std::ops::Mul;

use super::Vec4;
struct Mat4x4 {
    e: [[f64; 4]; 4],
}
impl Mat4x4 {
    fn construct<F>(element:F) -> Mat4x4
    where  
        F: Fn(usize, usize) -> f64
    {
        let mut e: [[f64; 4]; 4] = [[0.; 4]; 4];
        for r in 1..4 {
            for c in 1..4 {
                e[r][c] = element(r, c);
            }
        }
        Mat4x4{e}
    }
    /** 行列の`r`行目の行ベクトルを取り出す。 */
    fn row(&self, r:usize) -> Vec4 {
        Vec4::construct(|i| self.e[r][i])
    }
    /** 行列の`c`列目の列ベクトルを取り出す。 */
    fn col(&self, c:usize) -> Vec4 {
        Vec4::construct(|i| self.e[i][c])
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::construct(|r| self.row(r).dot(&rhs))
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        Mat4x4::construct(|r, c| self.row(r).dot(&rhs.col(c)))
    }
}
