use crate::{canvas::Canvas, mesh::VertexArrayObject, shader::pipeline::RenderingPipeline, util::{ClosedInterval, Vec4, Vec4Screen}};
use crate::shader::pipeline::Interpolable;

fn area(p0:&Vec4, p1:&Vec4, p2:&Vec4) -> f64 {
	let dx = p1 - p0;
	let dy = p2 - p0;
	dx.cross2d(&dy)
}
#[derive(Clone)]
pub struct MeshRenderer<R:RenderingPipeline> {
    culling: bool,
    _rp: R
}

impl<R> MeshRenderer<R> where R:RenderingPipeline {
    pub fn new(rp:R) -> Self {
        MeshRenderer { culling: false, _rp:rp }
    }
    pub fn render(
        &self,
        canvas:&mut Canvas,
        mesh:&VertexArrayObject<R::Attribute>,
        uni:&R::Uniform,
    ) {
        // 頂点シェーダー
        let vert_out:Vec<(<R as RenderingPipeline>::Varying, Vec4Screen)> =
            mesh.attribute.iter()
            .map(|attr| { R::vertex(uni, attr) } ).collect();
        let polygons =
            mesh.idx.iter()
            .map(|poly| { poly.map(|idx| { &vert_out[idx].0 }) });
        let varyings =
            mesh.idx.iter()
            .map(|poly| { poly.map(|idx| { &vert_out[idx].1.0 }) });
        for (varying, points) in polygons.zip(varyings) {
            // y基準でソート
            let (x_segment, y_segment, z_segment) = drawing_segments(canvas, &points);
            if z_segment.is_empty() { return; }
            
            // Barycentric座標
            let area_abc = area(&points[0], &points[1], &points[2]);
            
            // culling
            if self.culling && area_abc < 0. { return; }
            if area_abc.abs() < 1e-6 { return; }
            let inv_abc = 1./area_abc; 

            for y in &y_segment {
                for x in &x_segment {
                    let p = Vec4::newpixel(x, y);
                    let w = [
                        area(&points[1], &points[2], &p) * inv_abc,
                        area(&points[2], &points[0], &p) * inv_abc,
                        area(&points[0], &points[1], &p) * inv_abc,
                    ];
 
                    let u_intv = 0. .. 1.;
                    if  !w.iter().all(|e| u_intv.contains(e)) { continue; }

                    let p = p.to_point2();
                    let frag_vary = R::Varying::interpolate(&varying, &w);
                    let color = R::fragment(&p, uni, &frag_vary);
                    let depth = w[0]*points[0].z() + w[1]*points[1].z() + w[2]*points[2].z();
                    canvas.draw_pixel_with_depth(&p, &depth, &color);
                }
            }
        }
    }
}

fn drawing_segments(canvas: &mut Canvas, points: &[&Vec4; 3]) -> (ClosedInterval, ClosedInterval, ClosedInterval<f64>) {
    let bound_x = ClosedInterval::between(0,(canvas.width-1) as i32);
    let bound_y = ClosedInterval::between(0,(canvas.height-1) as i32);
    let bound_z = ClosedInterval::between(-1.,1.);
            
    let x_segment = ClosedInterval::range(points.iter().map(|p| p.x() as i32));
    let y_segment = ClosedInterval::range(points.iter().map(|p| p.y() as i32));
    let z_segment = ClosedInterval::range(points.iter().map(|p| p.z()));
            
    let x_segment = x_segment.and(&bound_x);
    let y_segment = y_segment.and(&bound_y);
    let z_segment = z_segment.and(&bound_z);
    (x_segment, y_segment, z_segment)
}