use crate::{canvas::Canvas, component::Transform, util::Mat4x4, world::camera::Camera};

pub trait Actor {
    fn update(&mut self, dt:f64) -> ();
    fn transform(&self) -> &Transform;
    fn render(&self, camera:&Camera, canvas:&mut Canvas, pv:&Mat4x4) -> ();
    fn is_terminated(&self) -> bool;
}
