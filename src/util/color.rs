use std::ops::{Deref, DerefMut};

use image::Rgba;
use glium::uniforms::{UniformType, UniformValue, AsUniformValue};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Color(Rgba<u8>);

impl Color {
	pub fn rgb(r: u8, g: u8, b: u8) -> Color {
		Color(Rgba { data: [r, g, b, 255] })
	}

	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color(Rgba { data: [r, g, b, a] })
	}
}

impl Default for Color {
	fn default() -> Color {
		Color(Rgba { data: [0, 0, 0, 255] })
	}
}

impl Deref for Color {
	type Target = Rgba<u8>;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Color {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.0
	}
}

impl AsUniformValue for Color {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Vec4([self[0] as f32 / 255.0,
		                    self[1] as f32 / 255.0,
		                    self[2] as f32 / 255.0,
		                    self[3] as f32 / 255.0])
	}

	fn matches(ty: &UniformType) -> bool {
		ty == &UniformType::FloatVec4
	}
}

impl From<(u8, u8, u8)> for Color {
	fn from((r, g, b): (u8, u8, u8)) -> Color {
		Color::rgb(r, g, b)
	}
}

impl From<(u8, u8, u8, u8)> for Color {
	fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
		Color::rgba(r, g, b, a)
	}
}

impl From<(u8, u8, u8, f32)> for Color {
	fn from((r, g, b, a): (u8, u8, u8, f32)) -> Color {
		Color::rgba(r, g, b, (a * 255) as u8)
	}
}

impl From<(f32, f32, f32)> for Color {
	fn from((r, g, b): (f32, f32, f32)) -> Color {
		Color::rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
	}
}

impl From<(f32, f32, f32, u8)> for Color {
	fn from((r, g, b, a): (f32, f32, f32, u8)) -> Color {
		Color::rgba((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, a)
	}
}

impl From<(f32, f32, f32, f32)> for Color {
	fn from((r, g, b, a): (u8, u8, u8, f32)) -> Color {
		Color::rgba((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8)
	}
}

impl<'a> From<&'a str> for Color {
	fn from(hex: &'a str) -> Color {
		let c = ::regex::Regex::new(r"^#([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})?$").unwrap().captures(hex).unwrap();
		let r = u8::from_str_radix(c.at(1).unwrap(), 16).unwrap();
		let g = u8::from_str_radix(c.at(2).unwrap(), 16).unwrap();
		let b = u8::from_str_radix(c.at(3).unwrap(), 16).unwrap();
		let a = c.at(4).map(|a| u8::from_str_radix(a, 16).unwrap()).unwrap_or(255);

		Color::rgba(r, g, b, a)
	}
}

impl<'a> From<(&'a str, u8)> for Color {
	fn from((hex, alpha): (&'a str, u8)) -> Color {
		let c = ::regex::Regex::new(r"^#([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})?$").unwrap().captures(hex).unwrap();
		let r = u8::from_str_radix(c.at(1).unwrap(), 16).unwrap();
		let g = u8::from_str_radix(c.at(2).unwrap(), 16).unwrap();
		let b = u8::from_str_radix(c.at(3).unwrap(), 16).unwrap();

		Color::rgba(r, g, b, alpha)
	}
}

impl<'a> From<(&'a str, f32)> for Color {
	fn from((hex, alpha): (&'a str, u8)) -> Color {
		let c = ::regex::Regex::new(r"^#([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})?$").unwrap().captures(hex).unwrap();
		let r = u8::from_str_radix(c.at(1).unwrap(), 16).unwrap();
		let g = u8::from_str_radix(c.at(2).unwrap(), 16).unwrap();
		let b = u8::from_str_radix(c.at(3).unwrap(), 16).unwrap();

		Color::rgba(r, g, b, (alpha * 255.0) as u8)
	}
}
