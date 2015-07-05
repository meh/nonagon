use ffmpeg::{frame, Rational};
use glium::{Display, Surface};

mod support;
pub use self::support::Support;

mod video;
pub use self::video::Video;

mod ship;
pub use self::ship::Ship;

use game;
use config;

pub struct Renderer<'a> {
	display: &'a Display,
	support: Support<'a>,

	video: Video<'a>,
	ship:  Ship<'a>,
}

impl<'a> Renderer<'a> {
	pub fn new<'b>(display: &'b Display, config: &config::Video, aspect: Rational) -> Renderer<'b> {
		Renderer {
			display: display,
			support: Support::new(display, config, aspect),

			video: Video::new(display),
			ship:  Ship::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.support.resize(width, height);
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, state: &game::State, frame: Option<&frame::Video>) {
		if let Some(frame) = frame {
			self.video.render(target, &self.support, frame);
		}

		self.ship.render(target, &self.support, &state.player);
	}
}
