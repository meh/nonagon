use ffmpeg::Rational;

use settings;
use analyzer::Analyzer;

pub struct Support<'s, 'a> {
	settings: &'s settings::Game,
	aspect:   Rational,
	tick:     usize,
	time:     f64,
	analyzer: &'a Analyzer,
}

impl<'s, 'a> Support<'s, 'a> {
	pub fn new(settings: &'s settings::Game, aspect: Rational, tick: usize, time: f64, analyzer: &'a Analyzer) -> Self {
		Support {
			settings: settings,
			aspect:   aspect,
			tick:     tick,
			time:     time,
			analyzer: analyzer,
		}
	}

	pub fn settings(&self) -> &settings::Game {
		self.settings
	}

	pub fn aspect(&self) -> Rational {
		self.aspect
	}

	pub fn tick(&self) -> usize {
		self.tick
	}

	pub fn time(&self) -> f64 {
		self.time
	}

	pub fn analyzer(&self) -> &Analyzer {
		self.analyzer
	}
}
