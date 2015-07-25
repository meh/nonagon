use glium::Surface;

use util::Color;
use renderer::{Support, Render};
use renderer::interface::{Text, Font};

pub struct Face<'te, 'ta, 's, 'f, S: Surface + 'static> {
	text:    &'te Text<'te>,
	target:  &'ta mut S,
	support: &'s Support<'s>,
	font:    &'f Font<'f>,

	color:   Color,
	size:    u32,
}

impl<'te, 'ta, 's, 'f, S: Surface + 'static> Face<'te, 'ta, 's, 'f, S> {
	pub fn new(text: &'te Text<'te>, target: &'ta mut S, support: &'s Support<'s>, font: &'f Font<'f>) -> Self {
		Face {
			text:    text,
			target:  target,
			support: support,
			font:    font,

			color:   Color::from("#000"),
			size:    1,
		}
	}

	pub fn color(mut self, color: &str) -> Self {
		self.color = Color::from(color);
		self
	}

	pub fn size(mut self, size: u32) -> Self {
		self.size = size;
		self
	}

	pub fn draw(&mut self, string: &str, x: u32, y: u32) {
		self.text.render(self.target, self.support, &(self.font, self.color, (x, y), self.size, string));
	}
}
