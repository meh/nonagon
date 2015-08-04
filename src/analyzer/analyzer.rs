use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::ops::{Deref, DerefMut};

use ffmpeg::{time, frame};

use super::{util, Beat};
use config;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Channel {
	Left(f64, Event),
	Right(f64, Event),
	Mono(f64, Event),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Event {
	Beat(f64),
}

pub struct Analyzer {
	receiver: Receiver<Channel>,
	sender:   Sender<frame::Audio>,

	start:     f64,
	timestamp: i64,
}

impl Analyzer {
	pub fn spawn(config: &config::Analyzer) -> Analyzer {
		let config = config.clone();

		let (event_sender, event_receiver) = channel::<Channel>();
		let (frame_sender, frame_receiver) = channel::<frame::Audio>();

		thread::spawn(move || {
			// The onset detector.
			let mut beat = Beat::new(config.window(),
				(config.threshold().size(), config.threshold().sensitivity()));

			// The buffer for the samples, so we can take out 1024 samples at a time.
			let mut samples = Vec::new();

			loop {
				// Get the next frame.
				let frame = ret!(frame_receiver.recv());

				// Add the samples to the pool.
				//
				// TODO: find a more performant way to do this.
				samples.extend(frame.plane::<i16>(0));

				// We need at least 1024 (2048 since it's stereo) samples before we can
				// analyze anything.
				if samples.len() < 2048 {
					continue;
				}

				// Separate into channels and run FFT on them and save them in the
				// windows.
				let (mono, left, right) = util::channels(&samples[0..2048]);

				// Drain the extracted samples.
				//
				// TODO: find a more performant way to do this.
				for _ in 0 .. 2048 {
					samples.remove(0);
				}

				// Send the mono channel to the onset detector.
				beat.analyze(&mono);

				// Check if we got a peak and send it as an event.
				if let Some((time, peak)) = beat.peak() {
					event_sender.send(Channel::Mono(time, Event::Beat(peak))).unwrap();
				}
			}
		});

		Analyzer {
			receiver: event_receiver,
			sender:   frame_sender,

			start:     0.0,
			timestamp: -1,
		}
	}

	pub fn start(&mut self, time: f64) {
		self.start = time;
	}

	pub fn time(&self) -> f64 {
		time::relative() as f64 / 1_000_000.0 - self.start
	}

	pub fn feed(&mut self, frame: frame::Audio) {
		if self.timestamp >= frame.timestamp().unwrap() {
			return;
		}

		self.timestamp = frame.timestamp().unwrap();
		self.sender.send(frame).unwrap();
	}
}

impl ::std::fmt::Debug for Analyzer {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		write!(f, "Analyzer {{ start: {} }}", self.start)
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
