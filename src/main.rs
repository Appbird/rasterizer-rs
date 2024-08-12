use log::error;
use error_iter::ErrorIter as _;
use fltk::app;
use fltk::prelude::*;
use fltk::window::Window;
use pixels::{Error, Pixels, SurfaceTexture};
const WIDTH:u32 = 640;
const HEIGHT:u32 = 480;

fn sq(x:i32) -> i32 { x * x }

fn main() -> Result<(), Error> {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(WIDTH as i32, HEIGHT as i32)
        .with_label("Hello Pixels");
    win.make_resizable(true);
    win.end();
    win.show();

    let mut pixels = {
        let pixel_width = win.pixel_w() as u32;
        let pixel_height = win.pixel_h() as u32;
        let surface_texture = SurfaceTexture::new(pixel_width, pixel_height, &win);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    }; 

    let radius = (WIDTH / 5) as i32;
    let (cx, cy) = ((WIDTH / 2) as i32, (HEIGHT / 2) as i32);

    while app.wait() {
        if let Err(err) = pixels.render() {
            panic!("{}", err);
        }
        for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;
            
            let rgba =
                if sq(x - cx) + sq(y - cy) < sq(radius) {
                    [0xff, 0xff, 0xff, 0xff]
                } else {
                    [0x72, 0x9f, 0xc2, 0xff]
                };
            
            pixel.copy_from_slice(&rgba);
        }
        app::flush();
        app::awake();
    }

    Ok(())
}
