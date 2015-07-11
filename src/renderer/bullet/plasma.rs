use glium::{Display, Surface};

use game;
use renderer::{Render, Support};

pub struct Plasma<'a> {
	display: &'a Display,
}

impl<'a> Plasma<'a>{
	pub fn new<'b>(display: &'b Display) -> Plasma<'b> {
		Plasma {
			display: display,
		}
	}
}

impl<'a> Render<game::Bullet> for Plasma<'a> {
	fn render<S: Surface>(&mut self, target: &mut S, support: &Support, state: &game::Bullet) {
		// uguu~
	}
}
