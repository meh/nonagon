use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::thread;
use std::mem;

use ffmpeg::{Error, Stream, format, frame, decoder, time};
use ffmpeg::channel_layout as layout;
use ffmpeg::format::sample;

use super::{Decoder, Reader};
use super::decoder::{get, try};

pub type D = super::Decoder<Details, frame::Audio>;

#[derive(Copy, Clone, Debug)]
pub struct Details {
	pub format:   format::Sample,
	pub rate:     u32,
	pub channels: u16,

	pub time_base: f64,
}

impl Details {
	pub fn from(codec: &decoder::Audio, stream: &Stream) -> Details {
		Details {
			format:   codec.format(),
			rate:     codec.rate(),
			channels: codec.channels(),

			time_base: stream.time_base().into(),
		}
	}
}

pub struct Audio {
	channel: Receiver<D>,
	details: Details,

	done:    bool,
	time:    i64,
	current: frame::Audio,
	next:    frame::Audio,
}

impl Audio {
	pub fn error(channel: &SyncSender<D>, error: Error) {
		channel.send(Decoder::Error(error)).unwrap();
	}

	pub fn none(channel: &SyncSender<D>) {
		channel.send(Decoder::Start(None)).unwrap();
	}

	pub fn spawn(mut codec: decoder::Audio, stream: &Stream, channel: SyncSender<D>) -> SyncSender<Reader> {
		channel.send(Decoder::Start(Some(Details::from(&codec, stream)))).unwrap();

		let (sender, receiver) = sync_channel(super::BOUND * 2);

		thread::spawn(move || {
			let mut decoded   = frame::Audio::empty();
			let mut resampler = codec.resampler(format::Sample::I16(sample::Type::Packed), layout::STEREO, 44100).unwrap();

			loop {
				match receiver.recv().unwrap() {
					Reader::Packet(packet) =>
						match codec.decode(&packet, &mut decoded) {
							Ok(true) => {
								let mut frame = frame::Audio::empty();
								frame.clone_from(&decoded);
								resampler.run(&decoded, &mut frame).unwrap();

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
		Audio {
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

	pub fn frame(&self) -> &frame::Audio {
		&self.current
	}

	pub fn sync(&mut self) -> f64 {
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

			0.0
		}
		else {
			pts - time
		}
	}
}
