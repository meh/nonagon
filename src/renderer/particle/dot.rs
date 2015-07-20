use glium::{Display, Surface};

use game;
use renderer::{Render, Support};

pub struct Dot<'a> {
	display: &'a Display,
}

impl<'a> Dot<'a>{
	pub fn new<'b>(display: &'b Display) -> Dot<'b> {
		Dot {
			display: display,
		}
	}
}

impl<'a> Render<game::Particle> for Dot<'a> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &Self::State) {
		// uguu~
	}
}
