use glium::{Display, Surface};

use game;
use renderer::{Render, Support};

pub struct Visualizer<'a> {
	display: &'a Display,
}

impl<'a> Visualizer<'a>{
	pub fn new<'b>(display: &'b Display) -> Visualizer<'b> {
		Visualizer {
			display: display,
		}
	}
}

impl<'a> Render<game::State> for Visualizer<'a> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &game::State) {
		// uguu~
	}
}
