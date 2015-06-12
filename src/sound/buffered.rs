use std::collections::VecDeque;
use std::ops::Deref;

use openal::{Error, Sample, Source, Buffer, source};

pub struct Buffered {
	source:  Source,
	buffers: VecDeque<source::Buffer>,
}

impl Buffered {
	pub fn new() -> Self {
		let mut source = Source::new();
		source.disable_looping();

		Buffered { source: source, buffers: VecDeque::new() }
	}

	pub fn play(&mut self) {
		if self.source.state() != source::State::Playing {
			self.source.play();
		}
	}

	pub fn queue<T: Sample>(&mut self, channels: u16, data: &[T], rate: u32) -> Result<usize, Error> {
		for _ in 0 .. self.source.processed() {
			self.buffers.pop_front();
		}

		self.buffers.push_back(self.source.queue(try!(Buffer::new(channels, data, rate))));

		Ok(self.buffers.len())
	}
}

impl Deref for Buffered {
	type Target = Source;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.source
	}
}
