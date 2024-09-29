use std::f64::consts::PI;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::util::{Color, Mat4x4, Throwable, Vec4, Vec4Project};

pub fn _lines(camera:&mut Camera, canvas:&mut Canvas, t:f64) -> Throwable<()> {
    for i in 0..50 {
        if i != (((t * 10.) as i32) % 50) { continue; }
        let theta = -PI/2.0 + (i as f64) * 2.0*PI / 50.0;
        let r = 1.0;
        let p1 = Vec4::new(0., 0., 0., 0.);
        let p2 = 
            &(r * &Vec4::new(f64::cos(theta), f64::sin(theta), 0., 0.)) + &p1;
        let p1 = Vec4Project(p1);
        let p2 = Vec4Project(p2);
        camera.draw_line(canvas, &p1, &p2, &Color::new(1., 1., 1., 1.));
    }
    Ok(())
}

pub fn _triangle(camera:&mut Camera, canvas:&mut Canvas) -> Throwable<()> {
    let points = [
        Vec4::new(0.5, 0.2, 0., 0.),
        Vec4::new(-0.8, -0.5, 0., 0.),
        Vec4::new(-0.5, 0.5, 0., 0.)
    ];
    let points:Vec<Vec4Project> = points.iter().map(|e| Vec4Project(e.clone())).collect();
    let white = Color::new(1., 1., 1., 1.);
    let red = Color::new(1., 0., 0., 1.);
    for point in &points {
        camera.draw_point(canvas, &point, &red);
    }
    camera.draw_triangle(
        canvas,
        &points[0], &points[1], &points[2],
        &white
    );
    Ok(())
}

pub fn affine(camera:&mut Camera, canvas:&mut Canvas, t:f64) -> Throwable<()> {
    let points = [
        Vec4::new(0.5, 0.25, 0., 1.),
        Vec4::new(0.5, 0.00, 0., 1.),
        Vec4::new(0.0, 0.00, 0., 1.),
        Vec4::new(0.0, 0.25, 0., 1.),
    ];
    let points:Vec<Vec4Project> = points.iter().map(|e| {
        let m =
            Mat4x4::translate(Vec4::new3d( -0.5, -0.25, 0.))
            * Mat4x4::rotation(Vec4::new3d(0., 0., 1.), PI*t/6.)
            * Mat4x4::scale(Vec4::new3d(0.8, 0.5, 1.));
        Vec4Project(m * e)
    }).collect();
    let white = Color::new(1., 1., 1., 1.);
    for point in &points {
        camera.draw_point(canvas, point, &white);
    }
    for (i, j) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
        let p1 = &points[i];
        let p2 = &points[j];
        camera.draw_line(canvas, p1, p2, &white);
    }
    Ok(())
}