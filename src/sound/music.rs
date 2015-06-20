use ffmpeg::frame;
use openal::source::{self, Source, Buffered};

pub struct Music {
	source:    Buffered,
	timestamp: i64,
}

impl Music {
	pub fn new() -> Self {
		let mut source = Source::new().buffered();
		source.disable_looping();

		Music {
			source:    source,
			timestamp: -1,
		}
	}

	pub fn play(&mut self, frame: &frame::Audio) {
		if self.timestamp >= frame.timestamp().unwrap() {
			return;
		}

		self.timestamp = frame.timestamp().unwrap();

		self.source.push(frame.channels(), frame.plane::<i16>(0), frame.rate()).unwrap();

		if self.source.state() != source::State::Playing {
			self.source.play();
		}
	}
}
