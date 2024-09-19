use std::cmp::{max, min};


pub struct ClosedInterval {
    min:i32,
    max:i32
}
pub type CInterval = ClosedInterval;
impl ClosedInterval {
    pub fn new() -> ClosedInterval{
        ClosedInterval{ min:0, max:0 }
    }
    pub fn between(a:i32, b:i32) -> ClosedInterval {
        ClosedInterval{ min: min(a, b), max: max(a, b) }   
    }
    pub fn and(self, i:ClosedInterval) -> ClosedInterval {
        ClosedInterval{
            min: max(self.min, i.min),
            max: min(self.max, i.max),
        }
    }
    pub fn or(self, i:ClosedInterval) -> ClosedInterval {
        ClosedInterval{
            min: min(self.min, i.min),
            max: max(self.max, i.max),
        }
    }
    pub fn empty(self) -> bool { self.min > self.max }
}