use std::sync::mpsc::Receiver;
use std::ops::Deref;

use ffmpeg::{Error, format, frame};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Details {
	pub format: format::Pixel,
	pub width:  u32,
	pub height: u32,
}

pub enum Data {
	Start(Option<Details>),
	Error(Error),
	Frame(frame::Video),
	End,
}

pub struct Video {
	channel: Receiver<Data>,
	details: Details,
}

impl Video {
	pub fn new(receiver: Receiver<Data>, details: Details) -> Self {
		Video {
			channel: receiver,
			details: details,
		}
	}

	pub fn format(&self) -> format::Pixel {
		self.details.format
	}

	pub fn width(&self) -> u32 {
		self.details.width
	}
	
	pub fn height(&self) -> u32 {
		self.details.height
	}
}

impl Deref for Video {
	type Target = Receiver<Data>;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.channel
	}
}
