use ffmpeg::frame;
use openal::{Error, Listener};
use openal::source::{self, Stream};

use game::State;
use config;

pub struct Sound<'a> {
	config: config::Audio,

	music:     Option<Stream<'a>>,
	timestamp: i64,

	listener: Listener<'a>,
}

impl<'a> Sound<'a> {
	pub fn new(config: &config::Audio) -> Result<Self, Error> {
		Ok(Sound {
			config: config.clone(),

			music:     None,
			timestamp: -1,

			listener: try!(Listener::default(&Default::default())),
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
