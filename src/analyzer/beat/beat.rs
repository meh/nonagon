use num::Complex;
use rft;

use analyzer::Band;
use super::{SpectralFlux, Threshold, State};
use settings;

#[derive(Debug)]
pub struct Beat {
	settings: settings::Analyzer,

	band:      Vec<Band>,
	spectral:  Vec<SpectralFlux>,
	threshold: Vec<Threshold>,
	state:     Vec<State>,
}

impl Beat {
	pub fn new(settings: &settings::Analyzer) -> Self {
		let mut bands      = Vec::new();
		let mut spectrals  = Vec::new();
		let mut thresholds = Vec::new();
		let mut states     = Vec::new();

		// If we have no bands just analyze the whole spectrum.
		if settings.beat().bands().is_empty() {
			let size = rft::spectrum::index_for(44100 / 2, settings.window().size(), 44100);

			bands.push(Band::new::<&str>(
				None, 0, 44100 / 2));

			spectrals.push(SpectralFlux::new(
				size));

			thresholds.push(Threshold::new(
				settings.beat().threshold().size(), settings.beat().threshold().sensitivity()));

			states.push(State::new(
				settings.beat().threshold().size()));
		}
		else {
			// Get the smallest low.
			let min = settings.beat().bands().iter().map(|b| b.range().start).min().unwrap();

			// Get the biggest high.
			let max = settings.beat().bands().iter().map(|b| b.range().end).max().unwrap();

			// If the first band doesn't include the zero frequency.
			if !settings.beat().ignore_missing() && min > 0 {
				let start = rft::spectrum::index_for(0, settings.window().size(), 44100);
				let end   = rft::spectrum::index_for(min, settings.window().size(), 44100);

				// Check there actually are frequencies in there.
				if end - start > 0 {
					bands.push(Band::new::<&str>(
						None, 0, min));

					spectrals.push(SpectralFlux::new(
						end - start));

					thresholds.push(Threshold::new(
						settings.beat().threshold().size(), settings.beat().threshold().sensitivity()));

					states.push(State::new(
						settings.beat().threshold().size()));
				}
			}

			for band in settings.beat().bands() {
				let start = rft::spectrum::index_for(band.range().start, settings.window().size(), 44100);
				let end   = rft::spectrum::index_for(band.range().end, settings.window().size(), 44100);

				bands.push(Band::new(
					band.name(), band.range().start, band.range().end));

				spectrals.push(SpectralFlux::new(
					end - start));

				thresholds.push(Threshold::new(
					band.threshold().size(), band.threshold().sensitivity()));

				states.push(State::new(
					band.threshold().size()));
			}

			// If the last band doesn't include the nyquist frequency.
			if !settings.beat().ignore_missing() && max < 44100 / 2 {
				let start = rft::spectrum::index_for(max, settings.window().size(), 44100);
				let end   = rft::spectrum::index_for(44100 / 2, settings.window().size(), 44100);

				// Check there actually are frequencies in there.
				if end - start > 0 {
					bands.push(Band::new::<&str>(
						None, max, 44100 / 2));

					spectrals.push(SpectralFlux::new(
						end - start));

					thresholds.push(Threshold::new(
						settings.beat().threshold().size(), settings.beat().threshold().sensitivity()));

					states.push(State::new(
						settings.beat().threshold().size()));
				}
			}
		}

		Beat {
			settings: settings.clone(),

			band:      bands,
			spectral:  spectrals,
			threshold: thresholds,
			state:     states,
		}
	}

	pub fn analyze(&mut self, input: &[Complex<f64>]) -> Vec<(f64, Band, f64)> {
		let mut result = Vec::new();

		let spectrum = rft::spectrum::compute(input);

		let band      = self.band.iter();
		let spectral  = self.spectral.iter_mut();
		let threshold = self.threshold.iter_mut();
		let state     = self.state.iter_mut();

		for (((band, spectral), threshold), state) in band.zip(spectral).zip(threshold).zip(state) {
			// Get the start as index for the spectrum.
			let start = rft::spectrum::index_for(band.low(), self.settings.window().size(), 44100);

			// Get the end as index for the spectrum.
			let end = rft::spectrum::index_for(band.high(), self.settings.window().size(), 44099);

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
					let time = (1.0 / 44100.0) * ((offset - 1) as f64 * self.settings.window().size() as f64);

					// Normalize the flux with the threshold.
					let flux = state.previous - threshold;

					// Add the peak.
					result.push((time, band.clone(), flux));
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
