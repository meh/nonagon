use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::ops::{Deref, DerefMut};

use ffmpeg::frame;

use super::{util, Beat};

#[derive(Debug)]
pub enum Channel {
	Left(f64, Event),
	Right(f64, Event),
	Mono(f64, Event),
}

#[derive(Debug)]
pub enum Event {
	Beat(f64),
}

pub struct Analyzer {
	receiver: Receiver<Channel>,
	sender:   Sender<frame::Audio>,
}

impl Analyzer {
	pub fn spawn() -> Analyzer {
		let (event_sender, event_receiver) = channel::<Channel>();
		let (frame_sender, frame_receiver) = channel::<frame::Audio>();

		thread::spawn(move || {
			// The onset detector.
			//
			// 1024 samples for the spectral flux analyzer.
			//
			// 10 samples of look-ahead and look-behind for the threshold.
			// 1.5 sensitivity for the threshold.
			let mut beat = Beat::new(1024, (10, 1.5));

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
		}
	}

	pub fn feed(&mut self, frame: frame::Audio) {
		self.sender.send(frame).unwrap();
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
