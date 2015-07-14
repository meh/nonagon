use glium::{Display, Surface};

use game;
use renderer::Support;

pub struct Visualizer<'a> {
	display: &'a Display,
}

impl<'a> Visualizer<'a>{
	pub fn new<'b>(display: &'b Display) -> Visualizer<'b> {
		Visualizer {
			display: display,
		}
	}

	pub fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &game::State) {
		// uguu~
	}
}
