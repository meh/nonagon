use num::Complex;

use super::{SpectralFlux, Threshold};
use util::Ring;

#[derive(Debug)]
pub struct Beat {
	samples: usize,

	spectral:  SpectralFlux,
	threshold: Threshold,

	fluxes:   Ring<f64>,
	previous: f64,
	peak:     Option<(f64, f64)>,
}

impl Beat {
	pub fn new(samples: usize, (size, sensitivity): (usize, f64)) -> Self {
		Beat {
			samples: samples,

			spectral:  SpectralFlux::new(samples),
			threshold: Threshold::new(size, sensitivity),

			fluxes:   Ring::new(size + 1),
			previous: 0.0,
			peak:     None,
		}
	}

	pub fn analyze(&mut self, input: &[Complex<f64>]) {
		// Get the current flux.
		let flux = self.spectral.rising(input);

		// Cache the flux.
		self.fluxes.push(flux);

		// Update the threshold with the new flux.
		self.threshold.push(flux);

		// Check if we have enough sample windows.
		if !self.threshold.is_enough() {
			return;
		}

		let current             = *self.fluxes.front().unwrap();
		let (offset, threshold) = self.threshold.current();

		// We have an outlier!
		if current > threshold {
			// Is it a beat?
			if self.previous > current {
				// The beat was actually in the previous sample.
				let time = (1.0 / 44100.0) * ((offset - 1) as f64 * 1024.0);

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
	}

	pub fn peak(&mut self) -> Option<(f64, f64)> {
		self.peak.take()
	}
}
