use ffmpeg::frame;
use glium::{Display, Surface};

mod video;
use self::video::Video;

mod ship;
use self::ship::Ship;

use game::State;

pub struct Renderer<'a> {
	display: &'a Display,

	video: Video<'a>,
	ship:  Ship<'a>,
}

impl<'a> Renderer<'a> {
	pub fn new<'b>(display: &'b Display) -> Renderer<'b> {
		Renderer {
			display: display,

			video: Video::new(display),
			ship:  Ship::new(display),
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, state: &State, frame: Option<&frame::Video>) {
		if let Some(frame) = frame {
			self.video.render(target, frame);
		}
	}
}
