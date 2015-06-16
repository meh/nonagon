use std::ops::Deref;

use openal::{Error, Sample, Source, Buffer, source};

pub struct Buffered {
	source: Source,
}

impl Buffered {
	pub fn new() -> Self {
		let mut source = Source::new();
		source.disable_looping();

		Buffered { source: source }
	}

	pub fn play(&mut self) {
		if self.source.state() != source::State::Playing {
			self.source.play();
		}
	}

	pub fn queue<T: Sample>(&mut self, channels: u16, data: &[T], rate: u32) -> Result<usize, Error> {
		self.source.clear();
		self.source.push(channels, data, rate).unwrap();

		Ok(self.source.queued())
	}
}

impl Deref for Buffered {
	type Target = Source;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.source
	}
}
