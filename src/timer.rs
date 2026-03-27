use std::time::{Duration, Instant};

pub struct Timer {
    pub total: Duration,
    pub remaining: Duration,
    last_tick: Instant,
    pub running: bool,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            total: duration,
            remaining: duration,
            last_tick: Instant::now(),
            running: false,
        }
    }

    /// Advance the timer. Returns true if the timer just expired.
    pub fn tick(&mut self) -> bool {
        if !self.running {
            return false;
        }
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;

        if elapsed >= self.remaining {
            self.remaining = Duration::ZERO;
            self.running = false;
            return true;
        }
        self.remaining -= elapsed;
        false
    }

    pub fn start(&mut self) {
        self.last_tick = Instant::now();
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn resume(&mut self) {
        self.last_tick = Instant::now();
        self.running = true;
    }

    pub fn reset(&mut self, duration: Duration) {
        self.total = duration;
        self.remaining = duration;
        self.running = false;
    }

    /// Progress from 0.0 (just started) to 1.0 (done)
    pub fn progress(&self) -> f64 {
        if self.total.is_zero() {
            return 1.0;
        }
        1.0 - (self.remaining.as_secs_f64() / self.total.as_secs_f64())
    }

    /// Format as MM:SS
    pub fn display(&self) -> String {
        let secs = self.remaining.as_secs();
        let m = secs / 60;
        let s = secs % 60;
        format!("{:02}:{:02}", m, s)
    }
}
