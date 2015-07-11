use glium::{Display, Surface};

use game;
use renderer::{Render, Support};

pub struct Ray<'a> {
	display: &'a Display,
}

impl<'a> Ray<'a>{
	pub fn new<'b>(display: &'b Display) -> Ray<'b> {
		Ray {
			display: display,
		}
	}
}

impl<'a> Render<game::Bullet> for Ray<'a> {
	fn render<S: Surface>(&mut self, target: &mut S, support: &Support, state: &game::Bullet) {
		// uguu~
	}
}
