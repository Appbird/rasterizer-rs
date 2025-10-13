use palette::{hsv, FromColor, Srgb};

use crate::util::Vec4;

pub fn hsv2vec4(h:f64, s:f64, v:f64) -> Vec4 {
    let c = Srgb::from_color(hsv::Hsv::new(h, s, v));
    Vec4::new(c.red, c.green, c.blue, 1.)
}