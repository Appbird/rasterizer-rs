use crate::{canvas::line::Line, util::{ClosedInterval, Color, Vec4, Vec4Screen}};
use super::Canvas;

fn area(p0:&Vec4, p1:&Vec4, p2:&Vec4) -> f64 {
	let dx = p1 - p0;
	let dy = p2 - p0;
	dx.cross2d(&dy)
}

impl Canvas {
    pub fn draw_triangle(
        &mut self,
        points: [Vec4Screen; 3],
        color: &[Color; 3]
    ) {
        // y基準でソート
		let points = points.map(|p| p.0);
		let bound_x = ClosedInterval::between(0,(self.width-1) as i32);
		let bound_y = ClosedInterval::between(0,(self.height-1) as i32);
		let bound_z = ClosedInterval::between(-1.,1.);
		
		let y_segment = ClosedInterval::range(points.iter().map(|p| p.y() as i32));
		let z_segment = ClosedInterval::range(points.iter().map(|p| p.z()));
		if z_segment.and(&bound_z).is_empty() { return; }
		
		// 辺を求める
		let mut points_idx:[usize; 3] = [0, 1, 2];
		points_idx.sort_by(|a, b| points[a.clone()].y().partial_cmp(&points[b.clone()].y()).unwrap_or(std::cmp::Ordering::Equal));
		let [bottom, middle, top] = [
			points[points_idx[0]].to_point2(),
			points[points_idx[1]].to_point2(),
			points[points_idx[2]].to_point2()
		];
		let lines = [
			Line::new(bottom, middle),
			Line::new(middle, top),
			Line::new(bottom, top),
		];
		// Barycentric座標
		let area_abc = area(&points[0], &points[1], &points[2]);
		if area_abc.abs() < 1e-6 { return; }
		let inv_abc = 1./area_abc; 

		for y in &y_segment.and(&bound_y) {
			let edge = if y < middle.y { &lines[0] } else { &lines[1] };
			let i_edge1 = lines[2].across_y(y);
			let i_edge2 = edge.across_y(y);
			let x_segment = i_edge1.or(i_edge2);
			for x in &x_segment.and(&bound_x) {
				let p = Vec4::newpixel(x, y);
				let w0 = area(&points[1], &points[2], &p) * inv_abc;
				let w1 = area(&points[2], &points[0], &p) * inv_abc;
				let w2 = area(&points[0], &points[1], &p) * inv_abc;
				self.draw_pixel(&p.to_point2(),&(w0*&color[0] + w1 * &color[1] + w2 * &color[2]));
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