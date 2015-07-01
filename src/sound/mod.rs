use ffmpeg::frame;
use openal::{Error, Listener};
use openal::source::{self, Stream};

use game::State;

pub struct Sound<'a> {
	music:     Option<Stream<'a>>,
	timestamp: i64,

	listener: Listener<'a>,
}

impl<'a> Sound<'a> {
	pub fn new() -> Result<Self, Error> {
		Ok(Sound {
			listener: try!(Listener::default(&Default::default())),

			music:     None,
			timestamp: -1,
		})
	}

	pub fn play<'b>(&'b mut self, frame: &frame::Audio) {
		if let None = self.music {
			self.music = Some(self.listener.source().unwrap().stream());
		}

		if self.timestamp >= frame.timestamp().unwrap() {
			return;
		}

		self.timestamp = frame.timestamp().unwrap();

		if let Some(source) = self.music.as_mut() {
			source.push(frame.channels(), frame.plane::<i16>(0), frame.rate()).unwrap();

			if source.state() != source::State::Playing {
				source.play();
			}
		}
	}

	pub fn render(&mut self, state: &State) {

	}
}
