mod util;
mod canvas;
mod camera;
mod world;

use std::{thread, time::{self, Instant}};

use camera::Camera;
use canvas::Canvas;
use world::*;
use util::{Color, Throwable};

fn main() -> Throwable<()> {
    let w: usize = 640;
    let h: usize = 480;
    let mut canvas = Canvas::new(w, h)?;
    let mut camera = Camera::new();
    
    let start_instant = Instant::now();
    let mut previous_instant = start_instant;
    while canvas.update()? {
        let current_instant = Instant::now();
        let delta_time = ((current_instant - previous_instant).as_millis() as f64) / 1000.;;
        previous_instant = current_instant;

        let fps = 1.0 / delta_time;
        let time = ((current_instant - start_instant).as_millis() as f64) / 1000.;
        affine(&mut camera, &mut canvas, time)?;
        
        println!("{fps}");
        let waiting_time = time::Duration::from_millis(20);
        thread::sleep(waiting_time);
    }
    Ok(())
}
