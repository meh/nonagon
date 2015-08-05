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

	pub fn compute(&mut self, input: &[f64]) -> f64 {
		debug_assert_eq!(input.len(), self.size);

		let mut result = 0.0;

		for (current, previous) in input.iter().zip(self.previous.iter_mut()) {
			let mut value = *current - *previous;

			result    += (value + value.abs()) / 2.0;
			*previous  = *current;
		}

		result
	}
}
