use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::ops::{Deref, DerefMut};

use ffmpeg::frame;

use super::{util, SpectralFlux, Threshold};

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
	sender:   Sender<Vec<i16>>,
}

impl Analyzer {
	pub fn spawn() -> Analyzer {
		let (event_sender,  event_receiver)  = channel::<Channel>();
		let (sample_sender, sample_receiver) = channel::<Vec<i16>>();

		thread::spawn(move || {
			// The current sample number, needed to know where in the stream we are.
			let mut index = 0;

			// The spectral flux analyzer.
			let mut beat = SpectralFlux::new(1024);

			// The threshold for the fluxes (21 samples [about a second worth], and 1.5 sensitivity).
			//
			// This is needed so we can filter out noise from the actual beats.
			let mut threshold = Threshold::new(21, 1.5);

			// The buffer for the samples, so we can take out 1024 samples at a time.
			let mut samples = Vec::new();

			// The previous prunned flux, so we can detect beats.
			let mut previous = 0.0;

			loop {
				samples.extend(ret!(sample_receiver.recv()));

				// We need at least 1024 (2048 since it's stereo) samples before we can
				// analyze the music.
				if samples.len() < 2048 {
					continue;
				}

				// Separate into channels and run FFT on them.
				let (mono, left, right) = util::channels(&samples[0..2048]);

				// Get the current flux.
				let current = beat.rising(&mono);

				// Update the threshold with the new flux.
				threshold.push(current);

				// We have an outlier!
				if current > threshold.current() {
					if previous > current {
						// The beat was actually in the previous sample.
						let time = (index - 1024) as f64 / 44100.0;

						// Normalize the flux with the threshold.
						let flux = previous - threshold.current();

						// Send the beat event.
						event_sender.send(Channel::Mono(time, Event::Beat(flux))).unwrap();
					}

					// Set the previous so we can get a new beat.
					previous = current - threshold.current();
				}
				else {
					// Reset the previous to 0 so we can get a new beat.
					previous = 0.0;
				}

				// TODO: find a more performant way, probably using VecDeques and doing
				// partial merges or something.
				for _ in 0 .. 2048 {
					samples.remove(0);
				}

				index += 1024;
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
