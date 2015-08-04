use config;
use game::Events;
use ffmpeg::Rational;

pub struct Support<'c, 'e> {
	config: &'c config::Game,
	aspect: Rational,
	tick:   usize,
	time:   f64,
	events: &'e Events,
}

impl<'c, 'e> Support<'c, 'e> {
	pub fn new(config: &'c config::Game, aspect: Rational, tick: usize, time: f64, events: &'e Events) -> Self {
		Support {
			config: config,
			aspect: aspect,
			tick:   tick,
			time:   time,
			events: events,
		}
	}

	pub fn config(&self) -> &config::Game {
		self.config
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
