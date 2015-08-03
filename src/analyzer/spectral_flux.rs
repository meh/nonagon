use num::Complex;

#[derive(Debug)]
pub struct SpectralFlux {
	size: usize,
	last: Vec<f64>,
}

impl SpectralFlux {
	pub fn new(size: usize) -> Self {
		SpectralFlux {
			size: size,
			last: vec![0.0; size],
		}
	}

	pub fn rising(&mut self, input: &[Complex<f64>]) -> f64 {
		if input.len() != self.size {
			panic!("size mismatch: input={} size={}", input.len(), self.size);
		}

		let mut flux = 0.0;

		for (current, last) in input.iter().zip(self.last.iter_mut()) {
			let value = current.norm_sqr() - *last;

			if value > 0.0 {
				flux += value;
			}

			*last = current.norm_sqr();
		}

		flux
	}

	pub fn falling(&mut self, input: &[Complex<f64>]) -> f64 {
		if input.len() != self.size {
			panic!("size mismatch: input={} size={}", input.len(), self.size);
		}

		let mut flux = 0.0;

		for (current, last) in input.iter().zip(self.last.iter_mut()) {
			let value = current.norm_sqr() - *last;

			if value < 0.0 {
				flux += value;
			}

			*last = current.norm_sqr();
		}

		flux
	}

	pub fn full(&mut self, input: &[Complex<f64>]) -> f64 {
		if input.len() != self.size {
			panic!("size mismatch: input={} size={}", input.len(), self.size);
		}

		let mut flux = 0.0;

		for (current, last) in input.iter().zip(self.last.iter_mut()) {
			flux += current.norm_sqr() - *last;
			*last = current.norm_sqr();
		}

		flux
	}
}
