use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::ops::Deref;
use std::thread;

use ffmpeg::{Error, Rational, Stream, Packet, format, frame, decoder};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Details {
	pub format: format::Pixel,

	pub width:  u32,
	pub height: u32,

	pub time_base: Rational,
}

impl Details {
	pub fn from(codec: &decoder::Video, stream: &Stream) -> Details {
		Details {
			format: codec.format(),

			width:  codec.width(),
			height: codec.height(),

			time_base: stream.time_base(),
		}
	}
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
	pub fn error(channel: &SyncSender<Data>, error: Error) {
		channel.send(Data::Error(error)).unwrap();
	}

	pub fn none(channel: &SyncSender<Data>) {
		channel.send(Data::Start(None)).unwrap();
	}

	pub fn spawn(mut codec: decoder::Video, stream: &Stream, channel: SyncSender<Data>) -> SyncSender<Option<Packet>> {
		channel.send(Data::Start(Some(Details::from(&codec, stream)))).unwrap();

		let (sender, receiver) = sync_channel(super::BOUND * 2);

		thread::spawn(move || {
			let mut decoded   = frame::Video::empty();
			let     converter = codec.converter(format::Pixel::RGB24).unwrap();

			loop {
				match receiver.recv().unwrap() {
					Some(packet) =>
						match codec.decode(&packet, &mut decoded) {
							Ok(true) => {
								let mut frame = frame::Video::new(format::Pixel::RGB24, decoded.width(), decoded.height());
								frame.copy(&decoded);
								converter.run(&decoded.picture(), &mut frame.picture()).unwrap();

								channel.send(Data::Frame(frame)).unwrap();
							},

							Ok(false)  => (),
							Err(error) => channel.send(Data::Error(error)).unwrap(),
						},

					None =>
						break
				}
			}

			channel.send(Data::End).unwrap();

			// XXX: hack
			::std::mem::forget(channel);
		});

		sender
	}

	pub fn new(channel: Receiver<Data>, details: Details) -> Self {
		Video {
			channel: channel,
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

	pub fn time_base(&self) -> Rational {
		self.details.time_base
	}
}

impl Deref for Video {
	type Target = Receiver<Data>;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.channel
	}
}
