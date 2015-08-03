use util::Ring;

pub struct Threshold {
	fluxes:      Ring<f64>,
	sensitivity: f64,
}

impl Threshold {
	pub fn new(size: usize, sensitivity: f64) -> Self {
		Threshold {
			fluxes:      Ring::new(size),
			sensitivity: sensitivity,
		}
	}

	pub fn push(&mut self, flux: f64) {
		self.fluxes.push(flux);
	}

	pub fn current(&self) -> f64 {
		self.sensitivity * (self.fluxes.iter()
			.fold(0.0, |acc, &n| acc + n) / self.fluxes.len() as f64)
	}
}
