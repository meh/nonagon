use std::collections::HashMap;

use male::onset::Peak;

use super::{Channel, Event};
use settings::analyzer as settings;

pub type Result = ::std::result::Result<Vec<Peak<settings::Band>>, ()>;

pub struct Beats {
	settings: settings::Analyzer,
	peaks:    Vec<Peak<settings::Band>>,

	last:          f64,
	last_for_band: HashMap<u64, f64>,
}

impl Beats {
	pub fn new(settings: &settings::Analyzer) -> Self {
		Beats {
			settings: settings.clone(),
			peaks:    Vec::new(),

			last: 0.0,
			last_for_band: HashMap::new(),
		}
	}

	pub fn handle(&mut self, event: &Channel) {
		if let &Channel::Mono(a, Event::Beat(ref peak)) = event {
			match self.peaks.binary_search_by(|b| b.offset().partial_cmp(&a).unwrap()) {
				Ok(index) | Err(index) =>
					self.peaks.insert(index, peak.clone())
			}
		}
	}

	pub fn fetch(&mut self, now: f64) -> Result {
		let mut index = 0;

		while index < self.peaks.len() {
			index += 1;

			if now > self.peaks[index - 1].offset() {
				break;
			}
		}

		let mut result = Vec::new();

		for peak in self.peaks.drain(0 .. index) {
			let key = hash(&peak);

			// Check global throttiling.
			if peak.offset() - self.settings.beat().throttle() <= self.last {
				continue;
			}

			// Check local throttling.
			if peak.offset() - peak.band().throttle() <= *self.last_for_band.get(&key).unwrap_or(&0.0) {
				continue;
			}

			self.last = peak.offset();
			self.last_for_band.insert(key, peak.offset());

			result.push(peak);
		}

		if result.is_empty() {
			Err(())
		}
		else {
			Ok(result)
		}
	}
}

fn hash(peak: &Peak<settings::Band>) -> u64 {
	use std::hash::{Hash, Hasher, SipHasher};

	let mut hash = SipHasher::new();
	peak.band().name().hash(&mut hash);
	peak.band().low().hash(&mut hash);
	peak.band().high().hash(&mut hash);

	hash.finish()
}
