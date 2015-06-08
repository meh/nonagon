use std::collections::VecDeque;
use std::ops::Deref;
use std::marker::Reflect;

use openal::{Error, Source, Buffer, source};

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

	pub fn queue<T: Reflect + 'static>(&mut self, channels: u16, data: &[T], rate: u32) -> Result<(), Error> {
		for _ in 0 .. self.source.processed() {
			self.buffers.pop_front();
		}

		self.buffers.push_back(self.source.queue(try!(Buffer::new(channels, data, rate))));

		Ok(())
	}
}

impl Deref for Buffered {
	type Target = Source;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.source
	}
}
