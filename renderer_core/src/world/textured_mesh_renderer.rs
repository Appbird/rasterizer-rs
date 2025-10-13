use image::{ImageReader, Rgba32FImage};

use crate::{canvas::Canvas, shader::{fragment, vertex}, util::{ClosedInterval, Mat4x4, Vec4, Vec4Project, Vec4Screen}, world::{camera::Camera, mesh::TexturedMesh}};

fn area(p0:&Vec4, p1:&Vec4, p2:&Vec4) -> f64 {
	let dx = p1 - p0;
	let dy = p2 - p0;
	dx.cross2d(&dy)
}

#[derive(Clone)]
pub struct TextureMeshRenderer {
    texture: Rgba32FImage,
    culling: bool
}

impl TextureMeshRenderer {
    pub fn new() -> Self {
        let texture = ImageReader::open("resource/texture.jpg").unwrap().decode().unwrap();
        let texture = texture.to_rgba32f();
        Self { texture, culling: false }
    }
    pub fn render(
        &self,
        mesh:&TexturedMesh,
        _camera:&Camera,
        canvas:&mut Canvas,
        pv:&Mat4x4,
        model:&Mat4x4
    ) {
        let pvm = pv * model;
        let converted_vertex:Vec<Vec4Project> =
            mesh.vertices.iter()
            .map(|v| { vertex::default_vshader(&pvm, v) })
            .collect();
        let w = 
            converted_vertex.iter().map(|v| { v.w() }).collect();
        let converted_vertex = 
            converted_vertex.into_iter().map(|v| {v.into_screen(&canvas.size())}).collect();
        
        self.rasterize(
            mesh,
            &converted_vertex,
            &w,
            canvas
        );
    }
    fn rasterize(
        &self,
        mesh:&TexturedMesh,
        converted_vertex:&Vec<Vec4Screen>,
        z_values:&Vec<f64>,
        canvas:&mut Canvas
    ) {
        for idx in &mesh.v_idx {
            let points:Vec<&Vec4> = idx.iter().map(|d| { &converted_vertex[*d].0 } ).collect();
            let uvs:Vec<&Vec4> = idx.iter().map(|d| { &mesh.uv[*d] } ).collect();
            let z_value:Vec<f64> = idx.iter().map(|d| { z_values[*d] }).collect();
            // y基準でソート
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
                    let z = 1./(w[0]/z_value[0] + w[1]/z_value[1] + w[2]/z_value[2]);
                    let uv = uvs[0]*w[0]/z_value[0] + uvs[1]*w[1]/z_value[1] + uvs[2]*w[2]/z_value[2];
                    let uv = uv * z;
                    let color = fragment::texture_fshader(p, &uv, &self.texture);
                    canvas.draw_pixel_with_depth(&p, &depth, &color);
                }
            }
        }
    }
}