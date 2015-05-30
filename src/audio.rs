use std::mem;

use ffmpeg::{Error, frame, time};

use cpal::Voice;

use ::source::audio as source;

pub struct Audio<'a> {
	source: &'a source::Audio,
	done:   bool,

	current: frame::Audio,
	next:    frame::Audio,

	voice: Voice,
}

impl<'a> Audio<'a> {
	pub fn new<'b>(source: &'b source::Audio) -> Result<Audio<'b>, Error> {
		Ok(Audio {
			current: try!(frame(&source)),
			next:    try!(frame(&source)),

			source: source,
			done:   false,

			voice: Voice::new(),
		})
	}

	pub fn is_done(&self) -> bool {
		self.done
	}

	pub fn frame(&self) -> &frame::Audio {
		&self.current
	}

	pub fn sync(&mut self) {
		match try_frame(&self.source) {
			Some(Ok(frame)) =>
				self.next = frame,

			Some(Err(Error::Eof)) =>
				self.done = true,

			Some(Err(error)) =>
				debug!("{:?}", error),

			_ => ()
		}

		mem::swap(&mut self.current, &mut self.next);

		debug!("{:?} {:?}", time::current(), self.next.timestamp());
	}

	pub fn play(&mut self) {
	}
}

fn frame(source: &source::Audio) -> Result<frame::Audio, Error> {
	loop {
		match source.recv() {
			Ok(source::Data::Frame(frame)) =>
				return Ok(frame),

			Ok(source::Data::Error(error)) => {
				debug!("{:?}", error);
				continue;
			},

			Ok(source::Data::End) =>
				return Err(Error::Eof),

			_ =>
				return Err(Error::Bug)
		}
	}
}

fn try_frame(source: &source::Audio) -> Option<Result<frame::Audio, Error>> {
	match source.try_recv() {
		Ok(source::Data::Frame(frame)) =>
			Some(Ok(frame)),

		Ok(source::Data::Error(error)) =>
			Some(Err(error)),

		Ok(source::Data::End) =>
			Some(Err(Error::Eof)),

		_ =>
			None
	}
}
