mod point2;
mod vec3;

pub use point2::*;
pub use vec3::*;

pub fn in_range<T: PartialOrd>(begin: T, x: T, end: T) -> bool {
    begin <= x && x < end
}
pub type Throwable<T> = Result<T, Box<dyn std::error::Error>>;