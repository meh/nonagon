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

		let (sender, receiver) = sync_channel(super::BOUND * 2);

		thread::spawn(move || {
			let mut decoded   = frame::Video::empty();
			let mut converter = codec.converter(format::Pixel::RGB24).unwrap();

			loop {
				match receiver.recv().unwrap() {
					Reader::Packet(packet) =>
						match codec.decode(&packet, &mut decoded) {
							Ok(true) => {
								let mut frame = frame::Video::empty();
								frame.clone_from(&decoded);
								converter.run(&decoded, &mut frame).unwrap();

								channel.send(Decoder::Frame(frame)).unwrap();
							},

							Ok(false)  => (),
							Err(error) => channel.send(Decoder::Error(error)).unwrap(),
						},

					Reader::End(..) =>
						break
				}
			}

			channel.send(Decoder::End(channel.clone())).unwrap();
		});

		sender
	}

	pub fn new(channel: Receiver<D>, details: Details) -> Self {
		Video {
			done:    false,
			time:    time::relative(),
			current: get(&channel).unwrap(),
			next:    get(&channel).unwrap(),

			channel: channel,
			details: details,
		}
	}

	pub fn details(&self) -> &Details {
		&self.details
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
				match try(&self.channel) {
					Some(Ok(frame)) => {
						mem::swap(&mut self.current, &mut self.next);
						self.next = frame;
					},

					Some(Err(Error::Eof)) =>
						self.done = true,

					_ => ()
				}
			}
			else {
				break;
			}
		}
	}
}
