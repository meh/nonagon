use std::sync::mpsc::sync_channel;
use std::thread;
use log::LogLevel;

use ffmpeg::{format, media, frame, Error};

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

macro_rules! try {
	($expr:expr, $audio:ident, $video:ident) => (match $expr {
		::std::result::Result::Ok(val) => val,
		::std::result::Result::Err(err) => {
			$audio.send(audio::Data::Error(err.clone())).unwrap();
			$video.send(video::Data::Error(err.clone())).unwrap();
			return;
		}
	})
}

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
			let     context = try!(format::open(path.as_ref()), audio_sender, video_sender);
			let mut audio   = None;
			let mut video   = None;

			if log_enabled!(LogLevel::Debug) {
				format::dump(&context, 0, Some(&path));
			}
			
			if let Some(stream) = context.streams().find(|s| s.codec().medium() == media::Type::Audio) {
				let codec = try!(stream.codec().decoder().and_then(|c| c.audio()), audio_sender, video_sender);

				audio = Some((stream, codec))
			}

			if let Some(stream) = context.streams().find(|s| s.codec().medium() == media::Type::Video) {
				let codec = try!(stream.codec().decoder().and_then(|c| c.video()), audio_sender, video_sender);

				video = Some((stream, codec))
			}

			audio_sender.send(audio::Data::Start(audio.as_ref().map(|&(_, ref codec)|
				audio::Details {
					format: codec.format(),
				}
			))).unwrap();

			video_sender.send(video::Data::Start(video.as_ref().map(|&(_, ref codec)|
				video::Details {
					format: codec.format(),
					width:  codec.width(),
					height: codec.height(),
				}
			))).unwrap();

			let mut packet  = context.packet();
			let mut v_frame = frame::Video::new();
			let mut a_frame = frame::Audio::new();

			while packet.read().is_ok() {
				if let Some((ref stream, ref mut codec)) = video {
					if packet.stream() == *stream {
						match codec.decode(&packet, &mut v_frame) {
							Ok(true)   => video_sender.send(video::Data::Frame(v_frame.clone())).unwrap(),
							Ok(false)  => (),
							Err(error) => video_sender.send(video::Data::Error(error)).unwrap(),
						}
					}
				}

				if let Some((ref stream, ref mut codec)) = audio {
					if packet.stream() == *stream {
						match codec.decode(&packet, &mut a_frame) {
							Ok(true)   => audio_sender.send(audio::Data::Frame(a_frame.clone())).unwrap(),
							Ok(false)  => (),
							Err(error) => audio_sender.send(audio::Data::Error(error)).unwrap(),
						}
					}
				}
			}

			audio_sender.send(audio::Data::End).unwrap();
			video_sender.send(video::Data::End).unwrap();
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
