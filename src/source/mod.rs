use std::sync::mpsc::{SyncSender, sync_channel};
use std::thread;

use log::LogLevel;
use ffmpeg::{format, media, Error, Packet};

pub mod decoder;
pub use self::decoder::Decoder;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub const FRAMES:  usize = 8;
pub const PACKETS: usize = 64;

pub enum Reader {
	Packet(Packet),
	End(SyncSender<Reader>),
}

pub fn spawn(path: &str, no_video: bool) -> (Result<Option<Audio>, Error>, Result<Option<Video>, Error>) {
	let path = path.to_string();

	let (video_sender, video_receiver) = sync_channel(FRAMES);
	let (audio_sender, audio_receiver) = sync_channel(FRAMES);

	thread::spawn(move || {
		let mut context = match format::open(&path) {
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
		let audio = match context.streams().find(|s| s.codec().medium() == media::Type::Audio) {
			Some(ref stream) => {
				let codec = match stream.codec().decoder().and_then(|c| c.audio()) {
					Ok(codec) =>
						codec,

					Err(error) => {
						Audio::error(&audio_sender, error);
						return;
					}
				};

				Some((Audio::spawn(codec, &stream, audio_sender), stream.index()))
			},

			_ => {
				Audio::none(&audio_sender);

				None
			}
		};

		// video decoder
		let video = match context.streams().find(|s| s.codec().medium() == media::Type::Video) {
			Some(ref stream) if !no_video => {
				let codec = match stream.codec().decoder().and_then(|c| c.video()) {
					Ok(codec) =>
						codec,

					Err(error) => {
						Video::error(&video_sender, error);
						return;
					}
				};

				Some((Video::spawn(codec, &stream, video_sender), stream.index()))
			},

			_ => {
				Video::none(&video_sender);

				None
			}
		};

		for (stream, packet) in context.packets() {
			if let Some((ref channel, index)) = video {
				if stream.index() == index {
					ret!(channel.send(Reader::Packet(packet.clone())));
				}
			}

			if let Some((ref channel, index)) = audio {
				if stream.index() == index {
					ret!(channel.send(Reader::Packet(packet.clone())));
				}
			}
		}

		if let Some((ref channel, _)) = video {
			ret!(channel.send(Reader::End(channel.clone())));
		}

		if let Some((ref channel, _)) = audio {
			ret!(channel.send(Reader::End(channel.clone())));
		}
	});

	let audio = match audio_receiver.recv().unwrap() {
		Decoder::Start(None) =>
			Ok(None),

		Decoder::Start(Some(details)) =>
			Ok(Some(Audio::new(audio_receiver, details))),

		Decoder::Error(error) =>
			Err(error),

		_ =>
			Err(Error::Bug),
	};

	let video = match video_receiver.recv().unwrap() {
		Decoder::Start(None) =>
			Ok(None),

		Decoder::Start(Some(details)) =>
			Ok(Some(Video::new(video_receiver, details))),

		Decoder::Error(error) =>
			Err(error),

		_ =>
			Err(Error::Bug),
	};

	(audio, video)
}
