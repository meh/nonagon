use std::sync::mpsc::{SyncSender, Receiver};

use ffmpeg::Error;

pub enum Data<T, U> {
	Start(Option<T>),
	Error(Error),
	Frame(U),
	End(SyncSender<Data<T, U>>),
}

pub fn get<T, U>(channel: &Receiver<Data<T, U>>) -> Result<U, Error> {
	loop {
		match channel.recv() {
			Ok(Data::Frame(frame)) =>
				return Ok(frame),

			Ok(Data::Error(error)) => {
				debug!("{:?}", error);
				continue;
			},

			Ok(Data::End(..)) =>
				return Err(Error::Eof),

			_ =>
				return Err(Error::Bug)
		}
	}
}

pub fn try<T, U>(channel: &Receiver<Data<T, U>>) -> Option<Result<U, Error>> {
	match channel.try_recv() {
		Ok(Data::Frame(frame)) =>
			Some(Ok(frame)),

		Ok(Data::Error(error)) =>
			Some(Err(error)),

		Ok(Data::End(..)) =>
			Some(Err(Error::Eof)),

		_ =>
			None
	}
}
