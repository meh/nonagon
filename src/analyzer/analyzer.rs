use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::ops::{Deref, DerefMut};

use ffmpeg::frame;

use super::{util, SpectralFlux};

#[derive(Debug)]
pub enum Channel {
	Left(Event),
	Right(Event),
	Mono(Event),
}

#[derive(Debug)]
pub enum Event {
	None,
}

pub struct Analyzer {
	receiver: Receiver<Channel>,
	sender:   Sender<Vec<i16>>,
}

impl Analyzer {
	pub fn spawn() -> Analyzer {
		let (event_sender,  event_receiver)  = channel::<Channel>();
		let (sample_sender, sample_receiver) = channel::<Vec<i16>>();

		thread::spawn(move || {
			let mut beat    = SpectralFlux::new(1024);
			let mut samples = Vec::new();

			loop {
				samples.extend(ret!(sample_receiver.recv()));

				// we need at least 1024 (2048 since it's stereo) samples before we can
				// analyze the music
				if samples.len() < 2048 {
					continue;
				}

				// TODO: find a more performant way, probably using VecDeques and doing
				// partial merges or something
				for _ in 0 .. 2048 {
					samples.remove(0);
				}
			}
		});

		Analyzer {
			receiver: event_receiver,
			sender:   sample_sender,
		}
	}

	pub fn feed(&mut self, frame: &frame::Audio) {
		self.sender.send(frame.plane::<i16>(0).to_owned()).unwrap();
	}
}

impl ::std::fmt::Debug for Analyzer {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		f.write_str("Analyzer")
	}
}

impl Deref for Analyzer {
	type Target = Receiver<Channel>;

	fn deref(&self) -> &Self::Target {
		&self.receiver
	}
}

impl DerefMut for Analyzer {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.receiver
	}
}
