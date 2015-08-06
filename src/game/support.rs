use settings;
use game::Events;
use ffmpeg::Rational;

pub struct Support<'s, 'e> {
	settings: &'s settings::Game,
	aspect:   Rational,
	tick:     usize,
	time:     f64,
	events:   &'e Events,
}

impl<'s, 'e> Support<'s, 'e> {
	pub fn new(settings: &'s settings::Game, aspect: Rational, tick: usize, time: f64, events: &'e Events) -> Self {
		Support {
			settings: settings,
			aspect:   aspect,
			tick:     tick,
			time:     time,
			events:   events,
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

	pub fn events(&self) -> &Events {
		self.events
	}
}
