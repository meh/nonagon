use ffmpeg::frame;

use glium::{Display, Surface};

use state::State;

mod video;
use self::video::Video;

pub struct Renderer<'a> {
	display: &'a Display,

	video: Video<'a>,
}

impl<'a> Renderer<'a> {
	pub fn new<'b>(display: &'b Display) -> Renderer<'b> {
		Renderer {
			display: display,

			video: Video::new(display),
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, state: &State, frame: Option<&frame::Video>) {
		if let Some(frame) = frame {
			self.video.draw(target, frame);
		}
	}
}
