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
		let x_segment = ClosedInterval::range(points.iter().map(|p| p.x() as i32));
		let y_segment = ClosedInterval::range(points.iter().map(|p| p.y() as i32));
		let z_segment = ClosedInterval::range(points.iter().map(|p| p.z()));
		if z_segment.and(&bound_z).is_empty() { return; }
		
		// Barycentric座標
		let area_abc = area(&points[0], &points[1], &points[2]);
		// culling
        if self.culling && area_abc < 0. { return; }
        if area_abc.abs() < 1e-6 { return; }
		let inv_abc = 1./area_abc; 

		for y in &y_segment.and(&bound_y) {
			for x in &x_segment.and(&bound_x) {
				let p = Vec4::newpixel(x, y);
				let w = [
					area(&points[1], &points[2], &p) * inv_abc,
					area(&points[2], &points[0], &p) * inv_abc,
					area(&points[0], &points[1], &p) * inv_abc,
				];
                if  !(0. < w[0] && w[0] < 1. &&
                    0. < w[1] && w[1] < 1. &&
                    0. < w[2] && w[2] < 1.)
                { continue; }
				let z = [
					points[0].z(), points[1].z(), points[2].z(), 
				];
				let p = p.to_point2();
				let depth = w[0]*z[0] + w[1]*z[1] + w[2]*z[2];
				let color = &color[0]*w[0] + &color[1]*w[1] + &color[2]*w[2];
				self.draw_pixel_with_depth(&p, &depth, &color);
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