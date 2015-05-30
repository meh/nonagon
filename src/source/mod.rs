use std::sync::mpsc::sync_channel;
use std::thread;
use log::LogLevel;

use ffmpeg::{format, media, Error};

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
		let (video_sender, video_receiver) = sync_channel(BOUND);
		let (audio_sender, audio_receiver) = sync_channel(BOUND);

		thread::spawn(move || {
			let context = match format::open(path.as_ref()) {
				Ok(context) =>
					context,

				Err(error) => {
					Audio::error(&audio_sender, error);
					Video::error(&video_sender, error);

					return;
				}
			};

			if log_enabled!(LogLevel::Debug) {
				format::dump(&context, 0, Some(&path));
			}
			
			// audio decoder
			let audio = if let Some(stream) = context.streams().find(|s| s.codec().medium() == media::Type::Audio) {
				let codec = match stream.codec().decoder().and_then(|c| c.audio()) {
					Ok(codec) =>
						codec,

					Err(error) => {
						Audio::error(&audio_sender, error);
						return;
					}
				};

				Some((Audio::spawn(codec, &stream, audio_sender), stream))
			}
			else {
				Audio::none(&audio_sender);

				None
			};

			// video decoder
			let video = if let Some(stream) = context.streams().find(|s| s.codec().medium() == media::Type::Video) {
				let codec = match stream.codec().decoder().and_then(|c| c.video()) {
					Ok(codec) =>
						codec,

					Err(error) => {
						Video::error(&video_sender, error);
						return;
					}
				};

				Some((Video::spawn(codec, &stream, video_sender), stream))
			}
			else {
				Video::none(&video_sender);

				None
			};

			let mut packet = context.packet();

			while packet.read().is_ok() {
				if let Some((ref channel, ref stream)) = video {
					if packet.stream() == *stream {
						channel.send(Some(packet.clone())).unwrap();
					}
				}

				if let Some((ref channel, ref stream)) = audio {
					if packet.stream() == *stream {
						channel.send(Some(packet.clone())).unwrap();
					}
				}
			}

			if let Some((ref channel, _)) = video {
				channel.send(None).unwrap();

				// XXX: hack
				::std::mem::forget(channel);
			}

			if let Some((ref channel, _)) = audio {
				channel.send(None).unwrap();

				// XXX: hack
				::std::mem::forget(channel);
			}
		});

		let video = match video_receiver.recv().unwrap() {
			video::Data::Start(details) =>
				details.map(|d| Video::new(video_receiver, d)),

			video::Data::Error(error) =>
				return Err(error),

			_ =>
				return Err(Error::Bug),
		};

		let audio = match audio_receiver.recv().unwrap() {
			audio::Data::Start(details) =>
				details.map(|d| Audio::new(audio_receiver, d)),

			audio::Data::Error(error) =>
				return Err(error),

			_ =>
				return Err(Error::Bug),
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
