use std::sync::mpsc::{SyncSender, Receiver};

use ffmpeg::Error;

pub enum Decoder<T, U> {
	Start(Option<T>),
	Error(Error),
	Frame(U),
	End(SyncSender<Decoder<T, U>>),
}

pub fn get<T, U>(channel: &Receiver<Decoder<T, U>>) -> Result<U, Error> {
	loop {
		match channel.recv() {
			Ok(Decoder::Frame(frame)) =>
				return Ok(frame),

			Ok(Decoder::Error(error)) => {
				debug!("{:?}", error);
				continue;
			},

			Ok(Decoder::End(..)) =>
				return Err(Error::Eof),

			_ =>
				return Err(Error::Bug)
		}
	}
}

pub fn try<T, U>(channel: &Receiver<Decoder<T, U>>) -> Option<Result<U, Error>> {
	match channel.try_recv() {
		Ok(Decoder::Frame(frame)) =>
			Some(Ok(frame)),

		Ok(Decoder::Error(error)) =>
			Some(Err(error)),

		Ok(Decoder::End(..)) =>
			Some(Err(Error::Eof)),

		_ =>
			None
	}
}
