mod point2;
mod vec4;
mod interval;
mod mat4x4;
mod debug;
mod stopwatch;
mod random;
pub mod ease;
pub mod color;

pub use point2::*;
pub use vec4::*;
pub use interval::*;
pub use mat4x4::*;
pub use stopwatch::Stopwatch;

pub fn in_range<T: PartialOrd>(begin: T, x: T, end: T) -> bool {
    begin <= x && x < end
}

pub type Throwable<T> = Result<T, Box<dyn std::error::Error>>;