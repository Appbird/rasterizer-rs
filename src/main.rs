mod util;
mod canvas;
mod camera;
mod world;
mod actor;

use std::{thread, time::{self, Instant}};

use camera::Camera;
use canvas::Canvas;
use world::*;
use util::{Color, Throwable};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let aspect = (h as f64) / (w as f64);
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new(aspect);
    
    conversion_3d(&mut camera, &mut canvas)?;
    Ok(())
}
