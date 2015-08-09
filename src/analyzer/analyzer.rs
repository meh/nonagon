use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

use ffmpeg::{time, frame};
use male::{Window, Onset, Band};
use male::window::filter;

use analyzer::{beats, Beats, Channel, Event};
use settings::analyzer as settings;
use settings::analyzer::Filter;

pub struct Analyzer {
	settings: settings::Analyzer,

	receiver: Receiver<Channel>,
	sender:   Sender<frame::Audio>,

	start:     f64,
	timestamp: i64,

	beats: Beats,
}

impl Analyzer {
	pub fn spawn(settings: &settings::Analyzer) -> Analyzer {
		let (event_sender, event_receiver) = channel::<Channel>();
		let (frame_sender, frame_receiver) = channel::<frame::Audio>();

		{
			let settings = settings.clone();

			thread::spawn(move || {
				// The window handler.
				let mut window = Window::new(settings.window().size(), 44100)
					.with_hop(settings.window().hop());

				match settings.window().filter() {
					Filter::None => (),

					Filter::Hamming =>
						window = window.with_filter::<filter::Hamming, _>(..),
				}

				// The beat detector.
				let mut onset = Onset::new(&window);

				// Add bands from configuration.
				for band in settings.beat().bands().iter().cloned() {
					let low       = band.range().start;
					let high      = band.range().end;
					let threshold = (band.threshold().size(), band.threshold().sensitivity());

					onset = onset.with_band(Band::<()>::new(low, high).with(band), Some(threshold));
				}
				
				loop {
					// Get the next frame.
					let frame = ret!(frame_receiver.recv());

					// Push the frame to the window.
					window.push(frame.plane::<i16>(0));

					// Get the next FFT channels, if any.
					if let Ok(channels) = window.next() {
						// Send the mono channel to the onset detector and send any peak as
						// an event.
						for peak in onset.analyze(&channels.mono()) {
							if let Ok(peak) = peak {
								event_sender.send(Channel::Mono(peak.offset(), Event::Beat(peak))).unwrap();
							}
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

			beats: Beats::new(settings),
		}
	}

	pub fn settings(&self) -> &settings::Analyzer {
		&self.settings
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

	fn fetch(&mut self) {
		while let Ok(ref event) = self.receiver.try_recv() {
			self.beats.handle(event);
		}
	}

	pub fn beats(&mut self) -> beats::Result {
		let now = self.time();

		self.fetch();
		self.beats.fetch(now)
	}
}
