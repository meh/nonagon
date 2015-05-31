use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::thread;
use std::mem;

use ffmpeg::{Error, Stream, format, frame, decoder, time};

use super::{Data, Reader};

pub type D = Data<Details, frame::Video>;

#[derive(Copy, Clone, Debug)]
pub struct Details {
	pub format: format::Pixel,

	pub width:  u32,
	pub height: u32,

	pub time_base: f64,
}

impl Details {
	pub fn from(codec: &decoder::Video, stream: &Stream) -> Details {
		Details {
			format: codec.format(),

			width:  codec.width(),
			height: codec.height(),

			time_base: stream.time_base().into(),
		}
	}
}

pub struct Video {
	channel: Receiver<D>,
	details: Details,

	done:    bool,
	time:    i64,
	current: frame::Video,
	next:    frame::Video,
}

impl Video {
	pub fn error(channel: &SyncSender<D>, error: Error) {
		channel.send(Data::Error(error)).unwrap();
	}

	pub fn none(channel: &SyncSender<D>) {
		channel.send(Data::Start(None)).unwrap();
	}

	pub fn spawn(mut codec: decoder::Video, stream: &Stream, channel: SyncSender<D>) -> SyncSender<Reader> {
		channel.send(Data::Start(Some(Details::from(&codec, stream)))).unwrap();

		let (sender, receiver) = sync_channel(super::BOUND * 2);

		thread::spawn(move || {
			let mut decoded   = frame::Video::empty();
			let     converter = codec.converter(format::Pixel::RGB24).unwrap();

			loop {
				match receiver.recv().unwrap() {
					Reader::Packet(packet) =>
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

					Reader::End(..) =>
						break
				}
			}

			channel.send(Data::End(channel.clone())).unwrap();
		});

		sender
	}

	pub fn new(channel: Receiver<D>, details: Details) -> Self {
		Video {
			done:    false,
			time:    time::relative(),
			current: super::data::get(&channel).unwrap(),
			next:    super::data::get(&channel).unwrap(),

			channel: channel,
			details: details,
		}
	}

	pub fn is_done(&self) -> bool {
		self.done
	}

	pub fn frame(&self) -> &frame::Video {
		&self.current
	}

	pub fn sync(&mut self) -> f64 {
		loop {
			if self.done {
				return 0.0;
			}

			let time: f64 = (time::relative() - self.time) as f64 / 1_000_000.0;
			let pts:  f64 = self.next.timestamp().unwrap_or(0) as f64 * self.details.time_base;

			if time > pts {
				match super::data::try(&self.channel) {
					Some(Ok(frame)) => {
						mem::swap(&mut self.current, &mut self.next);
						self.next = frame;
					},

					Some(Err(Error::Eof)) =>
						self.done = true,

					_ =>
						return 0.0
				}
			}
			else {
				return pts - time;
			}
		}
	}
}
