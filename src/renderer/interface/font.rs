use std::collections::HashMap;
use std::io::Read;
use std::ops::Deref;

use glium::Display;
use glium::texture::Texture2d;

use image::{DynamicImage, GenericImage, Luma};

use bdf;

pub struct Font<'a> {
	display: &'a Display,
	font:    bdf::Font,

	rows:    u32,
	columns: u32,

	width:  u32,
	height: u32,

	table: HashMap<char, (u32, u32)>,
	sheet: Texture2d,
}

impl<'a> Font<'a> {
	pub fn load<T: Read>(display: &Display, stream: T) -> Font {
		let font = bdf::read(stream).unwrap();

		// sort the glyphs by codepoints so the image is at least debuggable
		let mut codepoints = font.glyphs().iter().map(|(c, _)| *c).collect::<Vec<_>>();
		codepoints.sort();

		// table for codepoints to position in the sheet
		let mut table = HashMap::new();

		let columns = 64;
		let rows    = (codepoints.len() as f32 / columns as f32).ceil() as u32;

		// create an image with `columns` characters per row
		let mut image = DynamicImage::new_luma8(
			font.bounds().width * columns,
			font.bounds().height * rows);

		{
			// use luma to reduce size
			let mut luma = image.as_mut_luma8().unwrap();

			// clear the image to white
			for y in 0 .. luma.height() {
				for x in 0 .. luma.width() {
					luma.put_pixel(x, y, Luma([255]));
				}
			}

			for (index, codepoint) in codepoints.iter().enumerate() {
				let glyph = font.glyphs().get(codepoint).unwrap();
				let x     = index as u32 % columns;
				let y     = index as u32 / columns;

				// fill with black the set pixels
				for ((xg, yg), p) in glyph.pixels().filter(|&(_, p)| p) {
					// account for bounding box X offset
					let x = (x * font.bounds().width + xg) as i32
						+ glyph.bounds().x;

					// account for bounding box Y offset
					let y = (y * font.bounds().height + yg) as i32
						+ (font.bounds().height - glyph.bounds().height) as i32
						- glyph.bounds().y + font.bounds().y;

					luma.put_pixel(x as u32, y as u32, Luma([0]));
				}

				// put the position and codepoint in the table
				table.insert(*codepoint, (x, y));
			}
		}

		Font {
			display: display,
			font:    font,

			width:  image.dimensions().0,
			height: image.dimensions().1,

			rows:    rows,
			columns: columns,

			table: table,
			sheet: Texture2d::new(display, image).unwrap(),
		}
	}

	pub fn columns(&self) -> u32 {
		self.columns
	}

	pub fn rows(&self) -> u32 {
		self.rows
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn coordinates(&self, codepoint: char, u: f64, v: f64) -> [f64; 2] {
		let (x, y) = if let Some(&(x, y)) = self.table.get(&codepoint) {
			(x as f64, y as f64)
		}
		else if let Some(&(x, y)) = self.table.get(&'\u{FFFD}') {
			(x as f64, y as f64)
		}
		else {
			return [0.0, 0.0];
		};

		let rows = self.rows as f64;
		let cols = self.columns as f64;

		[(u / cols) + (1.0 / cols) * x,
		 (v / rows) + (1.0 / rows) * (rows - y - 1.0)]
	}
}

impl<'a> AsRef<Texture2d> for Font<'a> {
	fn as_ref(&self) -> &Texture2d {
		&self.sheet
	}
}

impl<'a> Deref for Font<'a> {
	type Target = bdf::Font;

	fn deref(&self) -> &Self::Target {
		&self.font
	}
}
