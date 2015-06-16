use ffmpeg::frame;
use glium::{Display, Surface};

mod scene;
pub use self::scene::Scene;

mod video;
pub use self::video::Video;

mod ship;
pub use self::ship::Ship;

use game::{self, Position};

pub struct Renderer<'a> {
	display: &'a Display,
	width:   u32,
	height:  u32,

	video: Video<'a>,
	ship:  Ship<'a>,
}

impl<'a> Renderer<'a> {
	pub fn new<'b>(display: &'b Display) -> Renderer<'b> {
		Renderer {
			display: display,

			width:  0,
			height: 0,

			video: Video::new(display),
			ship:  Ship::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.width  = width;
		self.height = height;
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, state: &game::State, frame: Option<&frame::Video>) {
		let scene = Scene::new(self.width, self.height);

		if let Some(frame) = frame {
			self.video.render(target, &scene, frame);
		}
	}
}
