

use crate::{snapshot, util::Point2};

/*
fn ceil_div(a:i32, b:i32) -> i32 {
	return (a + b - 1) / b;
}
*/

/**
 * 与えられたある2点の間を太さ1の線分で描いたときに通る画素の点を列挙するイテレータ。
 * 列挙方法はBresenhamの線分描画アルゴリズムに基づく。 
 * 線分の傾きをkとしたとき、-1 < k < 1の範囲で動作する。
 */
#[derive(Debug)]
 pub struct BresenhamLineIterAlongX {
	p1:Point2,
	 /** このイテレータがどの点を指しているかを示す。 */
	 current:Point2,
	 /** p2 - p1 */
	 offset: Point2,
	 /** {i32::signum(self.offset.x), i32::signum(self.offset.y)} */
	 sign_offset:Point2,
	 /** 現在描こうとしている点`current`と本来描こうとしている線分`Y`方向における累積誤差を表す。
	  * より正確には、この値に対してスケーリングと並行移動を施したものであり、こうすることで整数範囲内での演算に収まるように工夫されている。
	  * この値が0を超えるとy値にsが加算される。 */
	  err:i32,
}

impl BresenhamLineIterAlongX {
	pub fn new(p1:&Point2, p2:&Point2) -> Self {
		let offset = p2 - p1;
		BresenhamLineIterAlongX {
			p1: p1.clone(),
			current: p1.clone(),
			offset,
			sign_offset: Point2::new(i32::signum(offset.x), i32::signum(offset.y)),
			err: -offset.x,
		}
	}
	fn err_per_x_step(&self) -> i32 {
		return 2 * self.offset.y * self.sign_offset.y; // これは|offset.y| * sign_offset.xと等価
	}
	fn err_per_y_step(&self) -> i32 {
		return -2*self.offset.x *self.sign_offset.x;
	}
	/*
	pub fn y(&self, x:i32) -> i32 {
		let n = x - self.current.x;
		let err_per_x = self.err_per_x_step();
		let err_per_y = self.err_per_y_step();
		let required_step = (n*err_per_x - self.offset.x)/(-err_per_y);
		return self.current.y + required_step;
	}
	pub fn x(&self, y:i32) -> i32 {
		let m = y - self.current.y;
		let err_per_x = self.err_per_x_step();
		let err_per_y = self.err_per_y_step();
		let required_step = ceil_div(m*-err_per_y - self.offset.x, err_per_x);
		return self.current.x + required_step;
	}
	*/
}

impl Iterator for BresenhamLineIterAlongX {
    type Item = Point2;
    fn next(&mut self) -> Option<Self::Item> {
		if self.current.x == self.p1.x + self.offset.x { return None; }
        self.current.x += self.sign_offset.x;
		self.err += self.err_per_x_step();
		if self.err > 0 {
			self.current.y += self.sign_offset.y;
			self.err += self.err_per_y_step();
		}
		return Some(self.current.clone());
    }
}


pub struct BresenhamLine{
	base_iter:BresenhamLineIterAlongX,
	flipped: bool
}

impl BresenhamLine {
    pub fn trace(p1:&Point2, p2:&Point2) -> BresenhamLine {
        let offset = p2 - p1;
		let (base_iter, flipped) =
			if offset.x.abs() >= offset.y.abs() {
				(BresenhamLineIterAlongX::new(p1, p2), false)
			} else {
				(BresenhamLineIterAlongX::new(&p1.flipped(), &p2.flipped()), true)
			};
		BresenhamLine { base_iter, flipped }
    }
	/*
	pub fn x(&self, y:i32) -> i32 {
		if self.flipped { self.base_iter.y(y) } else { self.base_iter.x(y) }
	}
	pub fn y(&self, x:i32) -> i32 {
		if self.flipped { self.base_iter.x(x) } else { self.base_iter.y(x) }
	}
	 */
}

impl Iterator for BresenhamLine {
    type Item = Point2;
    fn next(&mut self) -> Option<Self::Item> {
		let next= self.base_iter.next();
		if self.flipped {
			match next {
				Some(x) => Some(x.flipped()),
				None => None
			}
		} else {
			next
		}
    }
}
