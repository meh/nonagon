use glium::{Display, Surface};

use lzma;

use renderer::{Render, Support};
use renderer::interface::{Font, Text, Face};
use game;

const NORMAL: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
	"/assets/gohufont.bdf.lzma"));

const BOLD: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
	"/assets/gohufont.bold.bdf.lzma"));

pub struct Interface<'a> {
	display: &'a Display,
	text:    Text<'a>,

	normal: Font<'a>,
	bold:   Font<'a>,
}

impl<'a> Interface<'a> {
	pub fn new(display: &Display) -> Interface {
		Interface {
			display: display,
			text:    Text::new(display),

			normal: Font::load(display, lzma::read(NORMAL).unwrap()),
			bold:   Font::load(display, lzma::read(BOLD).unwrap()),
		}
	}

	pub fn face<'ta, 's, 'f, S: Surface + 'static>(&'a self, target: &'ta mut S, support: &'s Support<'s>, font: &'f Font<'f>) -> Face<'a, 'ta, 's, 'f, S> {
		Face::new(&self.text, target, support, font)
	}
}

impl<'a> Render<game::State> for Interface<'a> {
	fn render<S: Surface + 'static>(&self, target: &mut S, support: &Support, state: &Self::State) {
		let mut face = self.face(target, support, &self.normal)
			.color("#f00")
			.size(2);

		face.draw(&format!("min={:.0}ms max={:.0}ms avg={:.0}ms",
			support.debug().min_frame_time() * 1_000.0,
			support.debug().max_frame_time() * 1_000.0,
			support.debug().avg_frame_time() * 1_000.0),
		5, 30);
	}
}
