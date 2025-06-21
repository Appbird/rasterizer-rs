use super::base::Canvas;
use super::bresenham::BresenhamLine;
use crate::util::{ClosedInterval, Color, Point2};

pub struct Line {
	p1: Point2,
	p2: Point2
}

fn round_div(n:i64, d:i64) -> i64 {
    if d < 0 {
		round_div(-n, -d)
	} else if n >= 0 {
        (n + d/2) / d
    } else {
        -((-n + d/2) / d)
    }
}

impl Line {
	pub fn new(p1:Point2, p2:Point2) -> Line { Line{ p1, p2 } }
	pub fn delta(&self) -> Point2 { self.p2 - self.p1 }
	pub fn across_y(&self, y:i32) -> ClosedInterval {
		if self.p1.y == self.p2.y {
			return 
				if self.p1.y == 0 { ClosedInterval::between(self.p1.x, self.p2.x) }
				else { ClosedInterval::empty() }
		}
		let delta = self.delta();
		let delta_x = delta.x as i64;
		let delta_y = delta.y as i64;
		let y_y1 = (y - self.p1.y) as i64;
		let x0 = self.p1.x as i64 + round_div(delta_x*y_y1, delta_y);
		let x1 = self.p1.x as i64 + round_div(delta_x*(y_y1+1), delta_y);
		ClosedInterval::between(x0 as i32, x1 as i32)
	}
}

impl Canvas {
    /**
     * 点p1から点p2への線分をBresenhamの線分描画アルゴリズムによって描画する。
     * Color値のうち、Alpha値は無視される。
     */
    pub fn draw_line(&mut self, p1:&Point2, p2:&Point2, color: &Color) {
        for p in BresenhamLine::trace(p1, p2){
            self.draw_pixel(&p, color)
        }
    }
	
}