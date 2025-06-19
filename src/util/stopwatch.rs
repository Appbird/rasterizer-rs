use std::time::{Duration, Instant};

pub struct Stopwatch {
	start: Option<Instant>,
	elapsed: Duration,
	running: bool,
}

impl Stopwatch {
	pub fn new() -> Self {
		Stopwatch {
			start: None,
			elapsed: Duration::ZERO,
			running: false,
		}
	}

	pub fn start(&mut self) {
		if !self.running {
			self.start = Some(Instant::now());
			self.running = true;
		}
	}

	pub fn stop(&mut self) {
		if self.running {
			if let Some(s) = self.start {
				self.elapsed += s.elapsed();
			}
			self.running = false;
			self.start = None;
		}
	}

	pub fn reset(&mut self) {
		self.start = None;
		self.elapsed = Duration::ZERO;
		self.running = false;
	}

	pub fn elapsed(&self) -> Duration {
		if self.running {
			if let Some(s) = self.start {
				return self.elapsed + s.elapsed();
			}
		}
		self.elapsed
	}
	pub fn elapsed_as_sec(&self) -> f64 {
		(self.elapsed().as_millis() as f64) / 1000.
	}

	pub fn is_running(&self) -> bool {
		self.running
	}
}