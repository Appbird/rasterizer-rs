use crate::{canvas::Canvas, world::{actor::Actor, camera::Camera}};

pub struct World<T> where T:Actor {
    actors:Vec<T>
}
impl<T> World<T> where T:Actor {
    pub fn new() -> Self {
        World {
            actors: vec![]
        }
    }
    pub fn spawn(&mut self, actor: T) {
        self.actors.push(actor);
    }
    pub fn update(&mut self, deltatime:f64) {
        let mut terminated = Vec::<usize>::new();
        for (idx, actor) in self.actors.iter_mut().enumerate() {
            actor.update(deltatime);
            if actor.is_terminated() { terminated.push(idx); }
        }
        self.sweep_obj(terminated);
    }
    pub fn draw(&self, camera:&Camera, canvas:&mut Canvas) {
        let p = camera.perspective_conversion();
        let v = camera.view_conversion();
        let pv = p*v;
        for actor in &self.actors {
            actor.render(camera, canvas, &pv);
        }
    }
    fn sweep_obj(&mut self, mut terminated:Vec::<usize>) {
        while let Some(idx) = terminated.pop() {
            self.actors.swap_remove(idx);
        }
    }
}