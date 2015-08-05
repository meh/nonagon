use std::vec::Drain;

use analyzer::{Analyzer, Channel, Event, Range};

#[derive(Debug)]
pub struct Events {
	beats: Vec<(f64, (Range, f64))>,
}

impl Events {
	pub fn new() -> Self {
		Events {
			beats: Vec::new(),
		}
	}

	pub fn fetch(&mut self, analyzer: &mut Analyzer) {
		while let Ok(event) = analyzer.try_recv() {
			match event {
				Channel::Mono(a, Event::Beat(band, flux)) => {
					match self.beats.binary_search_by(|&(b, _)| b.partial_cmp(&a).unwrap()) {
						Ok(index) | Err(index) =>
							self.beats.insert(index, (a, (band, flux)))
					}
				},

				_ =>
					()
			}
		}
	}

	pub fn beats(&mut self, analyzer: &Analyzer) -> Drain<(f64, (Range, f64))> {
		let     now   = analyzer.time();
		let mut index = 0;

		while index < self.beats.len() {
			index += 1;

			if now > self.beats[index - 1].0 {
				break;
			}
		}

		self.beats.drain(0 .. index)
	}
}
