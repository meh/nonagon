use glium::{Display, Surface};

use game;
use renderer::{Support};

pub struct Visualizer<'a> {
	display: &'a Display,
}

impl<'a> Visualizer<'a>{
	pub fn new<'b>(display: &'b Display) -> Visualizer<'b> {
		Visualizer {
			display: display,
		}
	}

	// FIXME: using the state is no good unless it provides an interface to the analyzer
	pub fn render<S: Surface>(&mut self, target: &mut S, support: &Support, state: &game::State) {
		target.clear_color(0.3, 0.3, 0.3, 1.0);
	}
}
