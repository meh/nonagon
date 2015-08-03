use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::thread;

use ffmpeg::{Error, Stream, format, frame, decoder};
use ffmpeg::channel_layout as layout;
use ffmpeg::format::sample;

use super::{Decoder, Reader};
use super::decoder::{get};

pub type D = super::Decoder<Details, frame::Audio>;

#[derive(Copy, Clone, Debug)]
pub struct Details {
	pub format: format::Sample,

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

	done: bool,
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

		let (sender, receiver) = sync_channel(super::PACKETS);

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
		Audio {
			done: false,

			channel: channel,
			details: details,
		}
	}

	pub fn format(&self) -> format::Sample {
		self.details.format
	}

	pub fn rate(&self) -> u32 {
		self.details.rate
	}

	pub fn channels(&self) -> u16 {
		self.details.channels
	}

	pub fn is_done(&self) -> bool {
		self.done
	}

	pub fn next(&mut self) -> Option<frame::Audio> {
		get(&self.channel).unwrap()
	}
}
