use std::f64::consts::PI;

use crate::{util::{Vec4, Vec4Model}, world::actor::Mesh};

pub fn tetrahedron(vert_color:[Vec4; 4]) -> Mesh {
    let vectors = [
        Vec4::newpoint(0., 1., 0.),
        Vec4::newpoint(f64::cos(0.), -1./3., f64::sin(0.)),
        Vec4::newpoint(f64::cos(2.*PI/3.), -1./3., f64::sin(2.*PI/3.)),
        Vec4::newpoint(f64::cos(4.*PI/3.), -1./3., f64::sin(4.*PI/3.)),
    ];
    let indecies: Vec<[usize; 3]> = vec![
        [1, 2, 3],
        [0, 2, 3],
        [0, 1, 3],
        [0, 1, 2],
    ];
    let mut vertices: Vec<Vec4Model> = vec![];
    let mut colors:Vec<Vec4> = vec![];
    let mut v_idx_list: Vec<[usize; 3]> = vec![];
    for (idx, index_array) in indecies.into_iter().enumerate() {
        let tri_vertices = vec![
            &vectors[index_array[0]],
            &vectors[index_array[1]],
            &vectors[index_array[2]]
        ];
        let g = (&(tri_vertices[0] + tri_vertices[1]) + tri_vertices[2])  / 3.;
        for i in 0..3 {
            let v = &((tri_vertices[i] - &g) * 0.95) + &g;
            vertices.push(Vec4Model(v));
            colors.push(vert_color[index_array[i]].clone());
        }
        v_idx_list.push([0 + idx * 3, 1 + idx*3, 2 + idx*3]);
    };
    Mesh {
        vertices,
        colors,
        v_idx: v_idx_list
    }
}