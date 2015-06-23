use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc::TryRecvError::Empty;

use ffmpeg::Error;

pub enum Decoder<T, U> {
	Start(Option<T>),
	Error(Error),
	Frame(U),
	End(SyncSender<Decoder<T, U>>),
}

pub fn get<T, U>(channel: &Receiver<Decoder<T, U>>) -> Result<Option<U>, Error> {
	match channel.recv() {
		Ok(Decoder::Frame(frame)) =>
			Ok(Some(frame)),

		Ok(Decoder::End(..)) =>
			Ok(None),

		Ok(Decoder::Error(error)) =>
			Err(error),

		Ok(Decoder::Start(..)) =>
			Err(Error::Bug),

		Err(..) =>
			Err(Error::Bug)
	}
}

pub fn try<T, U>(channel: &Receiver<Decoder<T, U>>) -> Option<Result<Option<U>, Error>> {
	match channel.try_recv() {
		Ok(Decoder::Frame(frame)) =>
			Some(Ok(Some(frame))),

		Ok(Decoder::End(..)) =>
			Some(Ok(None)),

		Err(Empty) =>
			None,

		Ok(Decoder::Start(..)) =>
			Some(Err(Error::Bug)),

		Ok(Decoder::Error(error)) =>
			Some(Err(error)),

		Err(..) =>
			Some(Err(Error::Bug))
	}
}
