use ffmpeg::{frame, Rational};
use glium::{Display, Surface};

mod scene;
pub use self::scene::Scene;

mod video;
pub use self::video::Video;

mod ship;
pub use self::ship::Ship;

use game;

pub struct Renderer<'a> {
	display: &'a Display,

	width:  u32,
	height: u32,
	aspect: Rational,

	video: Video<'a>,
	ship:  Ship<'a>,
}

impl<'a> Renderer<'a> {
	pub fn new<'b>(display: &'b Display, aspect: Rational) -> Renderer<'b> {
		Renderer {
			display: display,

			width:  0,
			height: 0,
			aspect: aspect,

			video: Video::new(display),
			ship:  Ship::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.width  = width;
		self.height = height;
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, state: &game::State, frame: Option<&frame::Video>) {
		let scene = Scene::new(self.width, self.height, self.aspect);

		if let Some(frame) = frame {
			self.video.render(target, &scene, frame);
		}

		self.ship.render(target, &scene, &state.player);
	}
}
