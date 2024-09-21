mod util;
mod canvas;
mod camera;
mod world;

use std::time::Instant;

use camera::Camera;
use canvas::Canvas;
use world::*;
use util::{Color, Throwable};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new();
    
    let previous_instant = Instant::now();
    while canvas.update()? {
        let current_instant = Instant::now();
        let time = ((current_instant - previous_instant).as_millis() as f64) / 1000.;
        lines(&mut camera, &mut canvas, time)?
    }
    Ok(())
}
