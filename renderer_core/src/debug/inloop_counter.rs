use std::sync::{Arc, LazyLock, Mutex};


pub struct InLoopCounter<'a> {
    name:&'a str,
    positive:usize,
    negative:usize
}

impl<'a> InLoopCounter<'a> {
    pub fn new(name: &'a str) -> Self {
        InLoopCounter { name, positive: 0, negative: 0 }
    }
    pub fn count(&mut self, s:bool){
        if s { self.positive += 1;  } else { self.negative += 1; }
    }
    pub fn reset(&mut self) {
        self.positive = 0;
        self.negative = 0;
    }
    pub fn describe(&self) -> String {
        return format!("{}: {} / {}", self.name, self.positive, self.positive+self.negative);
    }
}

macro_rules! def_inloop_counter {
    ($name:ident) => {
        pub static $name: LazyLock<Arc<Mutex<InLoopCounter>>> =
            LazyLock::new(|| 
                Arc::new(
                    Mutex::new(InLoopCounter::new(stringify!($name)))
                )
    );

    }
}
