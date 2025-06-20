use crate::{canvas::line::Line, util::{ClosedInterval, Color, Point2}};
use super::Canvas;

fn area(p0:&Point2, p1:&Point2, p2:&Point2) -> f64 {
	(p1 - p0).cross(&(p2 - p0)) as f64
}

impl Canvas {
    pub fn draw_triangle(
        &mut self,
        points: &[Point2; 3],
        color: &[Color; 3]
    ) {
        // y基準でソート
        let mut indecies: [usize; 3] = [0, 1, 2];
        indecies.sort_by(|a, b| points[a.clone()].y.cmp(&points[b.clone()].y));
		let range_y = ClosedInterval::range(points.map(|p| p.y.clone()));
		let inv_abc = 1./(area(&points[0], &points[1], &points[2]) as f64);
		let [bottom, middle, top] = indecies.map(|i| points[i.clone()]);
		let lines = [
			Line::new(bottom, middle),
			Line::new(middle, top),
			Line::new(bottom, top),
		];
		for y in &range_y {
			let edge = if y < middle.y { &lines[0] } else { &lines[1] };
			let i_edge1 = lines[2].across_y(y);
			let i_edge2 = edge.across_y(y);
			let segment = i_edge1.or(i_edge2);
			for x in &segment {
				let p = Point2{x, y};
				let w0 = area(&points[1], &points[2], &p) * inv_abc;
				let w1 = area(&points[2], &points[0], &p) * inv_abc;
				let w2 = area(&points[0], &points[1], &p) * inv_abc;
				self.draw_pixel(&p,&(w0*&color[0] + w1 * &color[1] + w2 * &color[2]));
			}
		}
        /*
		let [bottom, middle, top] = indecies.map(|i| points[i.clone()]);
		let lines = [
			Line::new(*bottom, *middle),
			Line::new(*middle, *top),
			Line::new(*bottom, *top),
		];
			let range_y = ClosedInterval::range(points.map(|p| p.y));
		for y in range_y {
			let edge = if y < middle.y { &lines[0] } else { &lines[1] };
			let i_edge1 = lines[2].across_y(y);
			let i_edge2 = edge.across_y(y);
			let segment = i_edge1.or(i_edge2);
			for x in segment {
				self.draw_pixel(&Point2 { x, y }, color)
			}
		}
		*/
    }
}