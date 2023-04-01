use std::time::{Duration, Instant};

pub struct TimeTracker(Instant);

impl TimeTracker {
    pub fn new() -> Self {
        Self(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }

    pub fn milestone(&mut self, name: &str) {
        eprintln!("{}: {}ms", name, self.elapsed().as_millis());
        self.0 = Instant::now();
    }
}
