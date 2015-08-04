use std::ops::Deref;

use analyzer::{Analyzer, Channel, Event};

#[derive(Debug)]
pub struct Events {
	start: f64,
	queue: Vec<Channel>,
}

impl Events {
	pub fn new() -> Self {
		Events {
			start: 0.0,
			queue: Vec::new(),
		}
	}

	pub fn fetch(&mut self, analyzer: &mut Analyzer) {
		let high = analyzer.time();

		self.queue.retain(|e| match e {
			&Channel::Left(offset, _) =>
				offset > high,

			&Channel::Right(offset, _) =>
				offset > high,

			&Channel::Mono(offset, _) =>
				offset > high,
		});

		while let Ok(v) = analyzer.try_recv() {
			self.queue.push(v);
		}
	}
}

impl Deref for Events {
	type Target = Vec<Channel>;

	fn deref(&self) -> &Self::Target {
		&self.queue
	}
}
