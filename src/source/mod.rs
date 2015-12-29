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

/// How many frames to decode before the waiting for the consumer to use them.
pub const FRAMES: usize = 8;

/// How many packets to read before the reader waits for them to be consumed by
/// the decoder threads.
///
/// Note that the packet limit is shared between audio and video packets.
pub const PACKETS: usize = 64;

/// Possible values sent to the decoder threads.
pub enum Reader {
	/// A new incoming packet.
	Packet(Packet),

	/// The EOF packet.
	///
	/// Note that the sender is sent along so the receiver will be able to
	/// receive all still-standing incoming packets.
	End(SyncSender<Reader>),
}

/// Spawns a packet reader, an audio decoder and a video decoder.
pub fn spawn(path: &str, no_video: bool) -> (Result<Option<Audio>, Error>, Result<Option<Video>, Error>) {
	let path = path.to_owned();

	let (video_sender, video_receiver) = sync_channel(FRAMES);
	let (audio_sender, audio_receiver) = sync_channel(FRAMES);

	// This thread will try to open the given path with ffmpeg then it will loop
	// until there are no more packets in the file.
	//
	// It will spawn a video and audio decoder and send the appropriate packets
	// to the appropriate thread.
	thread::spawn(move || {
		// Try to open the file, returning in case of error.
		let mut context = match format::input(&path) {
			Ok(context) =>
				context,

			Err(error) => {
				Audio::error(&audio_sender, error);
				Video::error(&video_sender, error);

				return;
			}
		};

		if log_enabled!(LogLevel::Debug) {
			format::context::input::dump(&context, 0, Some(&path));
		}
		
		// Spawn the audio decoder.
		let audio = match context.streams().find(|s| s.codec().medium() == media::Type::Audio) {
			Some(ref stream) => {
				let codec = match stream.codec().decoder().audio() {
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

		// Spawn the video decoder.
		let video = match context.streams().find(|s| s.codec().medium() == media::Type::Video) {
			Some(ref stream) if !no_video => {
				let codec = match stream.codec().decoder().video() {
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

		// Iterate over the packets.
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

		// Send the EOF to the audio decoder.
		if let Some((ref channel, _)) = audio {
			ret!(channel.send(Reader::End(channel.clone())));
		}

		// Send the EOF to the video decoder.
		if let Some((ref channel, _)) = video {
			ret!(channel.send(Reader::End(channel.clone())));
		}
	});

	// Check the status of the audio decoder and create the wrapper with the
	// decoder details.
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

	// Check the status of the video decoder and create the wrapper with the
	// decoder details.
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
