use util::Ring;

#[derive(Debug)]
pub struct Threshold {
	size:        usize,
	sensitivity: f64,

	fluxes: Ring<f64>,
}

impl Threshold {
	pub fn new(size: usize, sensitivity: f64) -> Self {
		Threshold {
			size:        size,
			sensitivity: sensitivity,
			fluxes:      Ring::new(size * 2 + 1),
		}
	}

	pub fn push(&mut self, flux: f64) {
		self.fluxes.push(flux);
	}

	pub fn pop(&mut self) -> f64 {
		let average = self.fluxes.iter().fold(0.0, |acc, &n| acc + n)
			/ self.fluxes.len() as f64;

		self.fluxes.pop();

		self.sensitivity * average
	}

	pub fn is_enough(&self) -> bool {
		self.fluxes.len() >= self.size * 2 + 1
	}
}
