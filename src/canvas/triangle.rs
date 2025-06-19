use crate::canvas::line::Line;
use crate::{snapshot, assert_cond};
use crate::util::{ClosedInterval, Color, Point2};
use super::{bresenham::BresenhamLine, Canvas};

impl Canvas {
    pub fn draw_triangle(
        &mut self,
        points: &[&Point2; 3],
        color: &Color
    ) {
        // y基準でソート
        let mut indecies: [usize; 3] = [0, 1, 2];
        indecies.sort_by(|a, b| points[a.clone()].y.cmp(&points[b.clone()].y));
        let [bottom, middle, top] = indecies.map(|i| points[i.clone()]);
		let range_y = ClosedInterval::range(points.map(|p| p.y));
		let lines = [
			Line::new(*bottom, *middle),
			Line::new(*middle, *top),
			Line::new(*bottom, *top),
		];
		for y in range_y {
			let edge = if y < middle.y { &lines[0] } else { &lines[1] };
			let i_edge1 = lines[2].across_y(y);
			let i_edge2 = edge.across_y(y);
			let segment = i_edge1.or(i_edge2);
			for x in segment {
				self.draw_pixel(&Point2 { x, y }, color)
			}
		}
    }
}