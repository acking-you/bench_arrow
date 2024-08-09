use std::time::Instant;

pub mod arrow_stream_reader;
pub struct Timer {
    ins: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            ins: Instant::now(),
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("cost time:{:?}", self.ins.elapsed())
    }
}
