use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::thread;
use std::mem;

use ffmpeg::{Error, Stream, format, frame, decoder, time};

use super::{Decoder, Reader};
use super::decoder::{get, try};

pub type D = Decoder<Details, frame::Video>;

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
		channel.send(Decoder::Error(error)).unwrap();
	}

	pub fn none(channel: &SyncSender<D>) {
		channel.send(Decoder::Start(None)).unwrap();
	}

	pub fn spawn(mut codec: decoder::Video, stream: &Stream, channel: SyncSender<D>) -> SyncSender<Reader> {
		channel.send(Decoder::Start(Some(Details::from(&codec, stream)))).unwrap();

		let (sender, receiver) = sync_channel(super::PACKETS);

		thread::spawn(move || {
			let mut decoded   = frame::Video::empty();
			let mut converter = codec.converter(format::Pixel::BGRA).unwrap();

			loop {
				match receiver.recv().unwrap() {
					Reader::Packet(packet) =>
						match codec.decode(&packet, &mut decoded) {
							Ok(true) => {
								let mut frame = frame::Video::empty();
								frame.clone_from(&decoded);
								converter.run(&decoded, &mut frame).unwrap();

								ret!(channel.send(Decoder::Frame(frame)));
							},

							Ok(false) =>
								(),

							Err(Error::Eof) =>
								break,

							Err(error) =>
								ret!(channel.send(Decoder::Error(error))),
						},

					Reader::End(..) =>
						break
				}
			}

			ret!(channel.send(Decoder::End(channel.clone())));
		});

		sender
	}

	pub fn new(channel: Receiver<D>, details: Details) -> Self {
		Video {
			done:    false,
			time:    time::relative(),
			current: get(&channel).unwrap().unwrap(),
			next:    get(&channel).unwrap().unwrap(),

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

	pub fn is_done(&self) -> bool {
		self.done
	}

	pub fn frame(&self) -> &frame::Video {
		&self.current
	}

	pub fn sync(&mut self) {
		loop {
			if self.done {
				break;
			}

			let time: f64 = (time::relative() - self.time) as f64 / 1_000_000.0;
			let pts:  f64 = self.next.timestamp().unwrap_or(0) as f64 * self.details.time_base;

			if time > pts {
				if let Some(result) = try(&self.channel) {
					if let Some(frame) = result.unwrap() {
						mem::swap(&mut self.current, &mut self.next);
						self.next = frame;
					}
					else {
						self.done = true;
					}
				}
			}
			else {
				break;
			}
		}
	}
}
