use std::sync::mpsc::sync_channel;
use std::thread;
use log::LogLevel;

use ffmpeg::{format, media, frame, Error};

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

// TODO: find a proper number
pub const BOUND: usize = 50;

pub struct Source {
	pub audio: Option<Audio>,
	pub video: Option<Video>,
}

impl Source {
	pub fn new(path: String) -> Result<Self, Error> {
		format::register_all();

		let (video_sender, video_receiver) = sync_channel(BOUND);
		let (audio_sender, audio_receiver) = sync_channel(BOUND);

		thread::spawn(move || {
			let context = match format::open(path.as_ref()) {
				Ok(context) =>
					context,

				Err(error) => {
					audio_sender.send(audio::Data::Error(error.clone())).unwrap();
					video_sender.send(video::Data::Error(error)).unwrap();
					return;
				}
			};

			if log_enabled!(LogLevel::Debug) {
				format::dump(&context, 0, Some(&path));
			}
			
			// audio decoder
			let audio = if let Some(stream) = context.streams().find(|s| s.codec().medium() == media::Type::Audio) {
				let mut codec = match stream.codec().decoder().and_then(|c| c.audio()) {
					Ok(codec) =>
						codec,

					Err(error) => {
						audio_sender.send(audio::Data::Error(error)).unwrap();
						return;
					}
				};

				audio_sender.send(audio::Data::Start(Some(audio::Details {
					format: codec.format(),
				}))).unwrap();

				let (sender, receiver) = sync_channel(BOUND * 2);

				Some((stream, sender, thread::scoped(move || {
					let     sender = audio_sender;
					let mut frame  = frame::Audio::new();

					loop {
						match receiver.recv().unwrap() {
							Some(packet) =>
								match codec.decode(&packet, &mut frame) {
									Ok(true)   => sender.send(audio::Data::Frame(frame.clone())).unwrap(),
									Ok(false)  => (),
									Err(error) => sender.send(audio::Data::Error(error)).unwrap(),
								},

							None =>
								break
						}
					}

					sender.send(audio::Data::End).unwrap();
				})))
			}
			else {
				audio_sender.send(audio::Data::Start(None)).unwrap();

				None
			};

			// video decoder
			let video = if let Some(stream) = context.streams().find(|s| s.codec().medium() == media::Type::Video) {
				let mut codec = match stream.codec().decoder().and_then(|c| c.video()) {
					Ok(codec) =>
						codec,

					Err(error) => {
						video_sender.send(video::Data::Error(error)).unwrap();
						return;
					}
				};

				video_sender.send(video::Data::Start(Some(video::Details {
					format: codec.format(),
					width:  codec.width(),
					height: codec.height(),
				}))).unwrap();

				let (sender, receiver) = sync_channel(BOUND * 2);

				Some((stream, sender, thread::scoped(move || {
					let     sender = video_sender;
					let mut frame  = frame::Video::new();

					loop {
						match receiver.recv().unwrap() {
							Some(packet) =>
								match codec.decode(&packet, &mut frame) {
									Ok(true)   => sender.send(video::Data::Frame(frame.clone())).unwrap(),
									Ok(false)  => (),
									Err(error) => sender.send(video::Data::Error(error)).unwrap(),
								},

							None =>
								break
						}
					}

					sender.send(video::Data::End).unwrap();
				})))
			}
			else {
				video_sender.send(video::Data::Start(None)).unwrap();

				None
			};

			let mut packet = context.packet();

			while packet.read().is_ok() {
				if let Some((ref stream, ref channel, _)) = video {
					if packet.stream() == *stream {
						channel.send(Some(packet.clone())).unwrap();
					}
				}

				if let Some((ref stream, ref channel, _)) = audio {
					if packet.stream() == *stream {
						channel.send(Some(packet.clone())).unwrap();
					}
				}
			}

			if let Some((_, ref channel, _)) = video {
				channel.send(None).unwrap();
			}

			if let Some((_, ref channel, _)) = audio {
				channel.send(None).unwrap();
			}
		});

		let video = match video_receiver.recv().unwrap() {
			video::Data::Start(details) =>
				details.map(|d| Video::new(video_receiver, d)),

			video::Data::Error(error) =>
				return Err(error),

			_ =>
				return Err(Error::bug()),
		};

		let audio = match audio_receiver.recv().unwrap() {
			audio::Data::Start(details) =>
				details.map(|d| Audio::new(audio_receiver, d)),

			audio::Data::Error(error) =>
				return Err(error),

			_ =>
				return Err(Error::bug()),
		};

		Ok(Source { audio: audio, video: video })
	}

	pub fn audio(&self) -> Option<&Audio> {
		self.audio.as_ref()
	}

	pub fn video(&self) -> Option<&Video> {
		self.video.as_ref()
	}
}
