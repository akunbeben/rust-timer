use std::default::Default;
use std::fmt;
use std::time::{Duration, Instant};

pub struct Timer {
    pub start_time: Option<Instant>,
    pub latest_time: Option<Instant>,
    pub elapsed: Duration,
}

impl Default for Timer {
    fn default() -> Timer {
        Timer {
            start_time: None,
            latest_time: None,
            idle_time: None,
            elapsed: Duration::from_secs(0),
        }
    }
}

impl Timer {
    pub fn new() -> Timer {
        Default::default()
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.latest_time = self.start_time;
        self.start_time = None;
    }

    pub fn sum_idle(&mut self) {
        self.idle_time = self.idle_time;
    }

    pub fn start_timer() -> Timer {
        let mut tm = Timer::new();
        tm.start();
        tm
    }

    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(timer) => timer.elapsed() + self.elapsed,
            None => self.elapsed,
        }
    }

    pub fn elapsed_time(&self) -> f32 {
        let dur = self.elapsed();
        dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.0
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elapsed_time())
    }
}