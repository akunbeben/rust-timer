use std::default::Default;
use std::io::Write;
use std::{fmt, io};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start_time: Option<Instant>,
    elapsed: Duration,
}

impl Default for Timer {
    fn default() -> Timer {
        Timer {
            start_time: None,
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

    pub fn elapsed_ms(&self) -> f32 {
        let dur = self.elapsed();
        dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.0
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elapsed_ms())
    }
}

fn main() {
    let tm = Timer::start_timer();

    let _ = io::stdout().flush().unwrap();

    while tm.start_time.is_some() {
        println!("{:.1$}", tm, 2)
    }
}
