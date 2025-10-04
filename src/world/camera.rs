use crate::{
    canvas::Canvas, shader::{fragment, vertex}, util::{ClosedInterval, Color, Mat4x4, Vec4, Vec4Project, Vec4Screen}
};
use crate::world::actor::Actor;
use std::f64::consts::PI;


fn area(p0:&Vec4, p1:&Vec4, p2:&Vec4) -> f64 {
	let dx = p1 - p0;
	let dy = p2 - p0;
	dx.cross2d(&dy)
}

pub struct Camera {
    pub near:f64,
    pub far:f64,
    pub width:f64,
    pub height:f64,
	pub position:Vec4,
	pub up:Vec4,
	pub look:Vec4,
    
    pub culling: bool
}

impl Camera {
    pub fn new(aspect: f64) -> Camera {
        let near = 0.1;
        let width = 2. * near * f64::tan(PI /6.);
        Camera{
            near: 0.1,
            far: 100.,
            width,
            height: width * aspect,
			position: Vec4::newpoint(0., 0., 0.),
			look: Vec4::newvec(0., 0., -1.),
			up: Vec4::newvec(0., 1., 0.),
            culling: false
        }
    }
    pub fn snapshot(&mut self, canvas:&mut Canvas, actors:&Vec<Actor>) {
        let perspective = self.perspective_conversion();
        let view = self.view_conversion();
        let pv = perspective*view;
        for actor in actors {
            let model = actor.model_conversion();
            let pvm = &pv * &model;
            for polygon in &actor.polygons {
                let projected = [
                    vertex::default_vshader(&pvm, &polygon.vertices[0]).into_screen(&canvas.size()),
                    vertex::default_vshader(&pvm, &polygon.vertices[1]).into_screen(&canvas.size()),
                    vertex::default_vshader(&pvm, &polygon.vertices[2]).into_screen(&canvas.size()),
                ];
                self.draw_triangle(canvas, projected, &polygon.color);
            }
        }
    }
    pub fn perspective_conversion(&self) -> Mat4x4 {
        let n = self.near;
        let f = self.far;
        let w = self.width;
        let h = self.height;
		
        Mat4x4::from_array([
            [ n / w,    0.,     0.,     0.  ],
            [ 0.,       n / h,  0.,     0.  ],
            [ 0.,       0.,     -(f+n)/(f-n), -2.*(f*n)/(f-n)],
            [ 0.,       0.,     -1.,     0.  ],
        ])
    }
	pub fn view_conversion(&self) -> Mat4x4 {
		let ez = -&self.look;
		let ex = &self.up.cross3d(&ez).normalized3d();
		let ey = ez.cross3d(&ex);
        Mat4x4::transposed_basis(&ex, &ey, &ez) * Mat4x4::translate(&-(&self.position))
    }

    pub fn draw_triangle(
        &mut self,
        canvas:&mut Canvas,
        points: [Vec4Screen; 3],
        color: &[Color; 3]
    ) {
        // y基準でソート
		let points = points.map(|p| p.0);
		let bound_x = ClosedInterval::between(0,(canvas.width-1) as i32);
		let bound_y = ClosedInterval::between(0,(canvas.height-1) as i32);
		let bound_z = ClosedInterval::between(-1.,1.);
		
        let x_segment = ClosedInterval::range(points.iter().map(|p| p.x() as i32));
		let y_segment = ClosedInterval::range(points.iter().map(|p| p.y() as i32));
		let z_segment = ClosedInterval::range(points.iter().map(|p| p.z()));
		
        let x_segment = x_segment.and(&bound_x);
        let y_segment = y_segment.and(&bound_y);
        let z_segment = z_segment.and(&bound_z);
        if z_segment.is_empty() { return; }
		
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

                let u_intv = 0. .. 1.;
                if  !w.iter().all(|e| u_intv.contains(e)) { continue; }
				
                let z = [
					points[0].z(), points[1].z(), points[2].z(), 
				];
				let p = p.to_point2();
				let depth = w[0]*z[0] + w[1]*z[1] + w[2]*z[2];
				let color = &color[0]*w[0] + &color[1]*w[1] + &color[2]*w[2];
				let color = fragment::fog_fshader(
                    p, depth,
                    &color,
                    &canvas.background_color, 
                    self.far, self.near,
                    6.0
                );
                canvas.draw_pixel_with_depth(&p, &depth, &color);
			}
		}
    }
}
