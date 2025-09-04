use std::f64::consts::PI;

use crate::{util::Vec4, world::actor::Polygon};

pub fn tetrahedron(vert_color:[Vec4; 3]) -> Vec<Polygon> {
    let vectors = [
        Vec4::newpoint(0., 1., 0.),
        Vec4::newpoint(f64::cos(0.), -1./3., f64::sin(0.)),
        Vec4::newpoint(f64::cos(2.*PI/3.), -1./3., f64::sin(2.*PI/3.)),
        Vec4::newpoint(f64::cos(4.*PI/3.), -1./3., f64::sin(4.*PI/3.)),
    ];
    let indecies: [[usize; 3]; 4] = [
        [1, 2, 3],
        [0, 2, 3],
        [0, 1, 3],
        [0, 1, 2],
    ];
    let mut polygons: Vec<Polygon> = vec![];
    for index_array in indecies {
        let mut vertex = [
            vectors[index_array[0]].clone(),
            vectors[index_array[1]].clone(),
            vectors[index_array[2]].clone()
        ];
        let g = (&(&vertex[0] + &vertex[1]) + (&vertex[2]))  / 3.;
        for i in 0..3 {
            vertex[i] = &((&vertex[i] - &g) * 0.95) + &g;
        }
        polygons.push(
            Polygon {
                vertices: vertex,
                color: vert_color.clone()
            }
        );
    };
    polygons
}