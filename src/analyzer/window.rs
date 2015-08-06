use num::Complex;
use ffmpeg::frame;
use rft;
use strided::MutStrided;

use settings;

#[derive(Debug)]
pub struct Window {
	settings: settings::analyzer::Window,
	buffer:   Vec<i16>,
}

impl Window {
	/// Creates a new window.
	pub fn new(settings: &settings::analyzer::Window) -> Self {
		Window {
			settings: settings.clone(),
			buffer:   Vec::with_capacity(settings.size() * 2 + settings.hop() * 2),
		}
	}

	/// Push a frame into the window.
	pub fn push(&mut self, frame: &frame::Audio) {
		self.buffer.extend(frame.plane::<i16>(0));
	}

	/// Get the FFT applied to the next available samples split into channels.
	pub fn next(&mut self) -> Option<(Vec<Complex<f64>>, Vec<Complex<f64>>, Vec<Complex<f64>>)> {
		if self.buffer.len() < self.settings.size() * 2 {
			return None;
		}

		let (mono, left, right) = {
			// Get 2N samples since it's packed stereo.
			let samples = &mut self.buffer[0 .. self.settings.size() * 2];

			// Our samples are stereo packed signed shorts, so split the two channels.
			let (mut left, mut right) = samples.as_stride_mut().substrides2_mut();
		
			// The two channels are averaged to get a mono channel, this might not be the
			// best way to do mono, but it works.
			let mut mono = left.iter().zip(right.iter())
				.map(|(&l, &r)| ((l as i32 + r as i32) / 2) as i16)
				.collect::<Vec<i16>>();
		
			// Apply the hamming if it's enabled.
			if self.settings.is_hamming() {
				rft::window::hamming_on(&mut *mono);

				// We cannot apply the hamming in-place for left and right when we're
				// hopping.
				if self.settings.size() != self.settings.hop() {
					(rft::forward(&*mono),
				 	 rft::forward(&*rft::window::hamming(left)),
				 	 rft::forward(&*rft::window::hamming(right)))
				}
				else {
					rft::window::hamming_on(left.reborrow());
					rft::window::hamming_on(right.reborrow());

					(rft::forward(&*mono),
				 	 rft::forward(left),
				 	 rft::forward(right))
				}
			}
			else {
				(rft::forward(&*mono),
				 rft::forward(left),
				 rft::forward(right))
			}
		};

		// Drain the extracted samples.
		//
		// We drain the hop size so we can implement hopping.
		self.buffer.drain(0 .. self.settings.hop() * 2);

		Some((mono, left, right))
	}
}
