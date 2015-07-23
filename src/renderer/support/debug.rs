use std::collections::VecDeque;
use std::f64::INFINITY;

pub struct Debug {
	time:   f64,
	frames: VecDeque<f64>,
}

impl Debug {
	pub fn new() -> Debug {
		Debug {
			time:   0.0,
			frames: VecDeque::new(),
		}
	}

	pub fn update(&mut self, time: f64) {
		self.frames.push_back(time - self.time);

		if self.frames.len() > 60 {
			self.frames.pop_front();
		}

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
