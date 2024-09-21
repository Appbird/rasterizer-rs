use std::cmp::{max, min};

pub struct ClosedIntervalIter {
    current:i32,
    end:i32
}
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

impl Iterator for ClosedIntervalIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = current + 1;
        if self.end > current { Some(current) } else { None }
    }
}

impl IntoIterator for ClosedInterval {
    type Item = i32;
    type IntoIter = ClosedIntervalIter;
    fn into_iter(self) -> Self::IntoIter {
        return ClosedIntervalIter{ current: self.min, end: self.max };
    }

}