use cpal::Voice;

use ::source;

pub struct Audio<'a> {
	source: &'a source::Audio,
	voice:  Voice,
}

impl<'a> Audio<'a> {
	pub fn new<'b>(source: &'b source::Audio) -> Audio<'b> {
		Audio {
			source: source,
			voice:  Voice::new(),
		}
	}

	pub fn play(&mut self) {
		self.source.try_recv().unwrap();
	}
}
