use super::base::Canvas;
use super::bresenham::BresenhamLine;
use crate::{snapshot, util::{ClosedInterval, Color, Point2}};

pub struct Line {
	p1: Point2,
	p2: Point2
}

fn round_div(n:i32, d:i32) -> i32 {
    // d == 0 は注意！
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
		let delta = self.delta();
		let x0 = self.p1.x + round_div(delta.x*(y - self.p1.y), delta.y);
		let x1 = self.p1.x + round_div(delta.x*(y + 1 - self.p1.y), delta.y);
		ClosedInterval::between(x0, x1)
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