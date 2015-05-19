use std::sync::mpsc::Receiver;
use std::ops::Deref;

use ffmpeg::{Error, format, frame};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Details {
	pub format: format::Sample,
}

pub enum Data {
	Start(Option<Details>),
	Error(Error),
	Frame(frame::Audio),
	End,
}

pub struct Audio {
	channel: Receiver<Data>,
	details: Details,
}

impl Audio {
	pub fn new(receiver: Receiver<Data>, details: Details) -> Self {
		Audio {
			channel: receiver,
			details: details,
		}
	}

	pub fn format(&self) -> format::Sample {
		self.details.format
	}
}

impl Deref for Audio {
	type Target = Receiver<Data>;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.channel
	}
}
