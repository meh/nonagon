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
	start:   f64,
	current: frame::Video,
	next:    frame::Video,
}

impl Video {
	// Sends a specific decoder error to the channel, helps inference.
	#[doc(hidden)]
	pub fn error(channel: &SyncSender<D>, error: Error) {
		channel.send(Decoder::Error(error)).unwrap();
	}

	// Sends an empty decoder to the channel, helps inference.
	#[doc(hidden)]
	pub fn none(channel: &SyncSender<D>) {
		channel.send(Decoder::Start(None)).unwrap();
	}

	#[doc(hidden)]
	pub fn spawn(mut codec: decoder::Video, stream: &Stream, channel: SyncSender<D>) -> SyncSender<Reader> {
		channel.send(Decoder::Start(Some(Details::from(&codec, stream)))).unwrap();

		// We use a synchronized channel so we don't decode the whole file and clog
		// the memory.
		let (sender, receiver) = sync_channel(super::PACKETS);

		// This thread will loop receiving video packets from the packet reader
		// thread until there are no more packets in the video stream.
		//
		// Once a packet is received, it will be decoded to a video frame.
		//
		// In case of error the error is sent upstream.
		//
		// In case of success the frame will be converted to BGRA from its native
		// pixel format, this way it will be able to be streamed to a texture
		// directly.
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

	#[doc(hidden)]
	pub fn new(channel: Receiver<D>, details: Details) -> Self {
		Video {
			done:    false,
			start:   -1.0,
			current: get(&channel).unwrap().unwrap(),
			next:    get(&channel).unwrap().unwrap(),

			channel: channel,
			details: details,
		}
	}

	/// Gets the format of the source.
	pub fn format(&self) -> format::Pixel {
		self.details.format
	}

	/// Gets the width of the source.
	pub fn width(&self) -> u32 {
		self.details.width
	}

	/// Gets the height of the source.
	pub fn height(&self) -> u32 {
		self.details.height
	}

	/// Checks if the stream is over.
	pub fn is_done(&self) -> bool {
		self.done
	}

	/// Gets the current frame.
	pub fn frame(&self) -> &frame::Video {
		&self.current
	}

	/// Sets the synchronized start time.
	pub fn start(&mut self, time: f64) {
		self.start = time;
	}

	/// Synchronizes the source to get the current frame.
	pub fn sync(&mut self) {
		loop {
			if self.done {
				break;
			}

			// Get how much time has passed since the start in seconds.
			let time = time::relative() as f64 / 1_000_000.0 - self.start;

			// Normalize the timestamp with the time base.
			let pts = self.next.timestamp().unwrap_or(0) as f64 * self.details.time_base;

			// Synchronization is based on frame skipping.
			//
			// I know, it sucks, but it works, and besides, you should be busy
			// shooting things.
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
