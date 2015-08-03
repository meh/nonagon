use std::f64::INFINITY;
use util::Ring;

pub struct Debug {
	time:   f64,
	frames: Ring<f64>,
}

impl Debug {
	pub fn new() -> Debug {
		Debug {
			time:   INFINITY,
			frames: Ring::new(60),
		}
	}

	pub fn update(&mut self, time: f64) {
		self.frames.push(time - self.time);
		self.time = time;
	}

	pub fn min_frame_time(&self) -> f64 {
		self.frames.iter().fold(INFINITY, |acc, &n| if n < acc { n } else { acc })
	}

	pub fn max_frame_time(&self) -> f64 {
		self.frames.iter().fold(-INFINITY, |acc, &n| if n > acc { n } else { acc })
	}

	pub fn avg_frame_time(&self) -> f64 {
		self.frames.iter().fold(0.0, |acc, &n| acc + n) / self.frames.len() as f64
	}
}
