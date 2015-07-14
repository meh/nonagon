use config;
use ffmpeg::Rational;

pub struct Support {
	config: config::Game,
	aspect: Rational,
	tick:   usize,
	time:   f64,
}

impl Support {
	pub fn new(config: config::Game, aspect: Rational, tick: usize, time: f64) -> Self {
		Support {
			config: config,
			aspect: aspect,
			tick:   tick,
			time:   time,
		}
	}

	pub fn config(&self) -> &config::Game {
		&self.config
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
}
