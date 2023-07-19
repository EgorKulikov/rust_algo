use std::time::{Duration, Instant};

pub struct TimeTracker(Instant, bool);

impl TimeTracker {
    pub fn new() -> Self {
        Self(Instant::now(), true)
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }

    pub fn disable(&mut self) {
        self.1 = false;
    }

    pub fn milestone(&mut self, name: &str) {
        if self.1 {
            eprintln!("{}: {}ms", name, self.elapsed().as_millis());
        }
        self.0 = Instant::now();
    }
}
 impl Default for TimeTracker {
    fn default() -> Self {
        Self::new()
    }
 }
 