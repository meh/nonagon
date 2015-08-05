use util::Ring;

#[derive(Debug)]
pub struct State {
	pub fluxes:   Ring<f64>,
	pub previous: f64,
}

impl State {
	pub fn new(size: usize) -> Self {
		State {
			fluxes:   Ring::new(size + 1),
			previous: 0.0,
		}
	}
}
