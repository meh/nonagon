use num::Complex;

#[derive(Debug)]
pub struct SpectralFlux {
	size:     usize,
	previous: Vec<f64>,
}

impl SpectralFlux {
	pub fn new(size: usize) -> Self {
		SpectralFlux {
			size:     size,
			previous: vec![0.0; size],
		}
	}

	pub fn rising(&mut self, input: &[Complex<f64>]) -> f64 {
		debug_assert_eq!(input.len(), self.size, "input size and internal size must match");

		let mut result = 0.0;

		for (current, previous) in input.iter().zip(self.previous.iter_mut()) {
			let value = current.norm_sqr() - *previous;

			if value > 0.0 {
				result += value;
			}

			*previous = current.norm_sqr();
		}

		result
	}

	pub fn falling(&mut self, input: &[Complex<f64>]) -> f64 {
		debug_assert_eq!(input.len(), self.size, "input size and internal size must match");

		let mut result = 0.0;

		for (current, previous) in input.iter().zip(self.previous.iter_mut()) {
			let value = current.norm_sqr() - *previous;

			if value < 0.0 {
				result += value;
			}

			*previous = current.norm_sqr();
		}

		result
	}

	pub fn full(&mut self, input: &[Complex<f64>]) -> f64 {
		debug_assert_eq!(input.len(), self.size, "input size and internal size must match");

		let mut result = 0.0;

		for (current, previous) in input.iter().zip(self.previous.iter_mut()) {
			 result   += current.norm_sqr() - *previous;
			*previous  = current.norm_sqr();
		}

		result
	}
}
