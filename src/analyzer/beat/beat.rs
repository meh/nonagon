use std::collections::VecDeque;
use num::Complex;

use super::{SpectralFlux, Threshold};

#[derive(Debug)]
pub struct Beat {
	samples: usize,
	offset:  usize,

	spectral:  SpectralFlux,
	threshold: Threshold,

	fluxes:   VecDeque<f64>,
	previous: f64,
	peak:     Option<(f64, f64)>,
}

impl Beat {
	pub fn new(samples: usize, (size, sensitivity): (usize, f64)) -> Self {
		Beat {
			samples: samples,
			offset:  0,

			spectral:  SpectralFlux::new(samples),
			threshold: Threshold::new(size, sensitivity),

			fluxes:   VecDeque::new(),
			previous: 0.0,
			peak:     None,
		}
	}

	pub fn analyze(&mut self, input: &[Complex<f64>]) {
		// Get the current flux.
		let flux = self.spectral.rising(input);

		// Cache the fluxes.
		self.fluxes.push_back(flux);

		// Update the threshold with the new flux.
		self.threshold.push(flux);

		// Check if we have enough sample windows.
		if !self.threshold.is_enough() {
			return;
		}

		let current   = self.fluxes.pop_front().unwrap();
		let threshold = self.threshold.pop();

		// We have an outlier!
		if current > threshold {
			// Is it a beat?
			if self.previous > current {
				// The beat was actually in the previous sample.
				let time = (1.0 / 44100.0) * self.offset as f64;

				// Normalize the flux with the threshold.
				let flux = self.previous - threshold;

				// Add the peak.
				self.peak = Some((time, flux));
			}

			// Set the previous so we can get a new beat.
			self.previous = current - threshold;
		}
		else {
			// Reset the previous to 0 so we can get a new beat.
			self.previous = 0.0;
		}

		self.offset += self.samples;
	}

	pub fn peak(&mut self) -> Option<(f64, f64)> {
		self.peak.take()
	}
}
