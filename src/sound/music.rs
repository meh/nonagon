use ffmpeg::frame;

use super::Buffered;

pub struct Music {
	source:    Buffered,
	timestamp: i64,
}

impl Music {
	pub fn new() -> Self {
		Music {
			source:    Buffered::new(),
			timestamp: -1,
		}
	}

	pub fn play(&mut self, frame: &frame::Audio) {
		if self.timestamp >= frame.timestamp().unwrap() {
			return;
		}

		self.timestamp = frame.timestamp().unwrap();

		self.source.queue(frame.channels(), frame.plane::<i16>(0), frame.rate()).unwrap();
		self.source.play();
	}
}
