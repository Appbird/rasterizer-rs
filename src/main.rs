use fltk::app;
use fltk::prelude::{GroupExt, WidgetExt};
use fltk::window::Window;
const WIDTH:u32 = 640;
const HEIGHT:u32 = 480;

fn main() {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(WIDTH as i32, HEIGHT as i32)
        .with_label("Hello Pixels");
    win.make_resizable(true);
    win.end();
    win.show();

    app.run().unwrap();
    
}
