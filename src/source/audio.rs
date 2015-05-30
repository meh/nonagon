use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::ops::Deref;
use std::thread;

use ffmpeg::{Error, Rational, Packet, Stream, format, frame, decoder};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Details {
	pub format: format::Sample,

	pub time_base: Rational,
}

impl Details {
	pub fn from(codec: &decoder::Audio, stream: &Stream) -> Details {
		Details {
			format: codec.format(),

			time_base: stream.time_base(),
		}
	}
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
	pub fn error(channel: &SyncSender<Data>, error: Error) {
		channel.send(Data::Error(error)).unwrap();
	}

	pub fn none(channel: &SyncSender<Data>) {
		channel.send(Data::Start(None)).unwrap();
	}

	pub fn spawn(mut codec: decoder::Audio, stream: &Stream, channel: SyncSender<Data>) -> SyncSender<Option<Packet>> {
		channel.send(Data::Start(Some(Details::from(&codec, stream)))).unwrap();

		let (sender, receiver) = sync_channel(super::BOUND * 2);

		thread::spawn(move || {
			let mut frame = frame::Audio::empty();

			loop {
				match receiver.recv().unwrap() {
					Some(packet) =>
						match codec.decode(&packet, &mut frame) {
							Ok(true)   => channel.send(Data::Frame(frame.clone())).unwrap(),
							Ok(false)  => (),
							Err(error) => channel.send(Data::Error(error)).unwrap(),
						},

					None =>
						break
				}
			}

			channel.send(Data::End).unwrap();
		});

		sender
	}

	pub fn new(channel: Receiver<Data>, details: Details) -> Self {
		Audio {
			channel: channel,
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
