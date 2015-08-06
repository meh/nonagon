use num::Complex;
use rft;

use analyzer::Range;
use super::{SpectralFlux, Threshold, State};
use config;

#[derive(Debug)]
pub struct Beat {
	config: config::Analyzer,

	range:     Vec<Range>,
	spectral:  Vec<SpectralFlux>,
	threshold: Vec<Threshold>,
	state:     Vec<State>,
}

impl Beat {
	pub fn new(config: &config::Analyzer) -> Self {
		let mut range     = Vec::new();
		let mut spectral  = Vec::new();
		let mut threshold = Vec::new();
		let mut state     = Vec::new();

		// If we have no bands just analyze the whole spectrum.
		if config.beat().bands().is_empty() {
			range.push(Range::new(
				0, 44100 / 2));

			spectral.push(SpectralFlux::new(
				config.window().size()));

			threshold.push(Threshold::new(
				config.beat().threshold().size(), config.beat().threshold().sensitivity()));

			state.push(State::new(
				config.beat().threshold().size()));
		}
		else {
			// Get the smallest low.
			let min = config.beat().bands().iter().map(|b| b.range().low).min().unwrap();

			// Get the biggest high.
			let max = config.beat().bands().iter().map(|b| b.range().high).max().unwrap();

			// If the first band doesn't include the zero frequency.
			if !config.beat().ignore_missing() && min > 0 {
				let start = rft::spectrum::index_for(0, config.window().size(), 44100);
				let end   = rft::spectrum::index_for(min, config.window().size(), 44100);

				// Check there actually are frequencies in there.
				if end - start > 0 {
					range.push(Range::new(
						0, min));

					spectral.push(SpectralFlux::new(
						end - start));

					threshold.push(Threshold::new(
						config.beat().threshold().size(), config.beat().threshold().sensitivity()));

					state.push(State::new(
						config.beat().threshold().size()));
				}
			}

			for band in config.beat().bands() {
				let start = rft::spectrum::index_for(band.range().low, config.window().size(), 44100);
				let end   = rft::spectrum::index_for(band.range().high, config.window().size(), 44100);

				range.push(Range::new(
					band.range().low, band.range().high));

				spectral.push(SpectralFlux::new(
					end - start));

				threshold.push(Threshold::new(
					band.threshold().size(), band.threshold().sensitivity()));

				state.push(State::new(
					band.threshold().size()));
			}

			// If the last band doesn't include the nyquist frequency.
			if !config.beat().ignore_missing() && max < 44100 / 2 {
				let start = rft::spectrum::index_for(max, config.window().size(), 44100);
				let end   = rft::spectrum::index_for(44100 / 2, config.window().size(), 44100);

				// Check there actually are frequencies in there.
				if end - start > 0 {
					range.push(Range::new(
						max, 44100 / 2));

					spectral.push(SpectralFlux::new(
						end - start));

					threshold.push(Threshold::new(
						config.beat().threshold().size(), config.beat().threshold().sensitivity()));

					state.push(State::new(
						config.beat().threshold().size()));
				}
			}
		}

		Beat {
			config: config.clone(),

			range:     range,
			spectral:  spectral,
			threshold: threshold,
			state:     state,
		}
	}

	pub fn analyze(&mut self, input: &[Complex<f64>]) -> Vec<(f64, Range, f64)> {
		let mut result = Vec::new();

		let spectrum = rft::spectrum::compute(input);

		let range     = self.range.iter();
		let spectral  = self.spectral.iter_mut();
		let threshold = self.threshold.iter_mut();
		let state     = self.state.iter_mut();

		for (((&range, spectral), threshold), state) in range.zip(spectral).zip(threshold).zip(state) {
			// Get the start as index for the spectrum.
			let start = rft::spectrum::index_for(range.low, self.config.window().size(), 44100);

			// Get the end as index for the spectrum.
			let end = rft::spectrum::index_for(range.high, self.config.window().size(), 44100);

			// Compute the flux for the specified part of the spectrum.
			let flux = spectral.compute(&spectrum[start .. end]);

			// Cache the flux.
			state.fluxes.push(flux);

			// Update the threshold with the new flux.
			threshold.push(flux);

			// Check we have enough sample windows to calculate the threshold.
			if !threshold.is_enough() {
				continue;
			}

			let current             = *state.fluxes.front().unwrap();
			let (offset, threshold) = threshold.current();

			// We have an outlier!
			if current > threshold {
				// Is it a beat?
				if state.previous > current {
					// The beat was actually in the previous sample.
					let time = (1.0 / 44100.0) * ((offset - 1) as f64 * self.config.window().size() as f64);

					// Normalize the flux with the threshold.
					let flux = state.previous - threshold;

					// Add the peak.
					result.push((time, range, flux));
				}

				// Set the previous so we can get a new beat.
				state.previous = current - threshold;
			}
			else {
				// Reset the previous to 0 so we can get a new beat.
				state.previous = 0.0;
			}
		}

		result
	}
}
