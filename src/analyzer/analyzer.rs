use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::ops::{Deref, DerefMut};

use ffmpeg::{time, frame};

use super::{Window, Beat, Band};
use settings;

#[derive(PartialEq, Clone, Debug)]
pub enum Channel {
	Left(f64, Event),
	Right(f64, Event),
	Mono(f64, Event),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Event {
	Beat(Band, f64),
}

pub struct Analyzer {
	settings: settings::Analyzer,

	receiver: Receiver<Channel>,
	sender:   Sender<frame::Audio>,

	start:     f64,
	timestamp: i64,
}

impl Analyzer {
	pub fn spawn(settings: &settings::Analyzer) -> Analyzer {
		let (event_sender, event_receiver) = channel::<Channel>();
		let (frame_sender, frame_receiver) = channel::<frame::Audio>();

		{
			let settings = settings.clone();

			thread::spawn(move || {
				// The window handler.
				let mut window = Window::new(settings.window());

				// The beat detector.
				let mut beat = Beat::new(&settings);
				
				loop {
					// Get the next frame.
					let frame = ret!(frame_receiver.recv());

					// Push the frame to the window.
					window.push(&frame);

					// Get the next FFT channels, if any.
					if let Some((mono, left, right)) = window.next() {
						// Send the mono channel to the onset detector and send any peak as
						// an event.
						for &(time, ref band, peak) in &beat.analyze(&mono) {
							event_sender.send(Channel::Mono(time, Event::Beat(band.clone(), peak))).unwrap();
						}
					}
				}
			});
		}

		Analyzer {
			settings: settings.clone(),

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
