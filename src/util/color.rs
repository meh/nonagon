use std::ops::{Deref, DerefMut};

use image::Rgba;
use glium::uniforms::{UniformType, UniformValue, AsUniformValue};

#[derive(PartialEq, Eq, Clone, Copy)]
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

impl<T: Parse> From<T> for Color {
	fn from(value: T) -> Color {
		Parse::parse(value).unwrap()
	}
}

impl ::std::fmt::Display for Color {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		f.write_str(&format!("#{:02x}{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2], self.0[3]))
	}
}


impl ::std::fmt::Debug for Color {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
		::std::fmt::Display::fmt(self, f)
	}
}

pub trait Parse {
	fn parse(value: Self) -> Result<Color, &'static str>;
}

impl Parse for (u8, u8, u8) {
	fn parse((r, g, b): (u8, u8, u8)) -> Result<Color, &'static str> {
		Ok(Color::rgb(r, g, b))
	}
}

impl Parse for (u8, u8, u8, u8) {
	fn parse((r, g, b, a): (u8, u8, u8, u8)) -> Result<Color, &'static str> {
		Ok(Color::rgba(r, g, b, a))
	}
}

impl Parse for (u8, u8, u8, f32) {
	fn parse((r, g, b, a): (u8, u8, u8, f32)) -> Result<Color, &'static str> {
		if a < 0.0 || a > 1.0 {
			Err("value out of range")
		}
		else {
			Ok(Color::rgba(r, g, b, (a * 255.0) as u8))
		}
	}
}

impl Parse for (f32, f32, f32) {
	fn parse((r, g, b): (f32, f32, f32)) -> Result<Color, &'static str> {
		if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
			Err("value out of range")
		}
		else {
			Ok(Color::rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8))
		}
	}
}

impl Parse for (f32, f32, f32, u8) {
	fn parse((r, g, b, a): (f32, f32, f32, u8)) -> Result<Color, &'static str> {
		if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
			Err("value out of range")
		}
		else {
			Ok(Color::rgba((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, a))
		}
	}
}

impl Parse for (f32, f32, f32, f32) {
	fn parse((r, g, b, a): (f32, f32, f32, f32)) -> Result<Color, &'static str> {
		if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 || a < 0.0 || a > 1.0 {
			Err("value out of range")
		}
		else {
			Ok(Color::rgba((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8))
		}
	}
}

impl<'a> Parse for &'a str {
	fn parse(string: &'a str) -> Result<Color, &'static str> {
		let (r, g, b, a) = try!(rgba(string));

		Ok(Color::rgba(r, g, b, a))
	}
}

impl<'a> Parse for (&'a str, u8) {
	fn parse((string, alpha): (&'a str, u8)) -> Result<Color, &'static str> {
		let (r, g, b, _) = try!(rgba(string));

		Ok(Color::rgba(r, g, b, alpha))
	}
}

impl<'a> Parse for (&'a str, f32) {
	fn parse((string, alpha): (&'a str, f32)) -> Result<Color, &'static str> {
		if alpha < 0.0 || alpha > 1.0 {
			Err("value out of range")
		}
		else {
			let (r, g, b, _) = try!(rgba(string));

			Ok(Color::rgba(r, g, b, (alpha * 255.0) as u8))
		}
	}
}

fn rgba(string: &str) -> Result<(u8, u8, u8, u8), &'static str> {
	use std::iter;

	if let Some(c) = ::regex::Regex::new(r"^#([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})?$").unwrap().captures(string) {
		let r = u8::from_str_radix(c.at(1).unwrap(), 16).unwrap();
		let g = u8::from_str_radix(c.at(2).unwrap(), 16).unwrap();
		let b = u8::from_str_radix(c.at(3).unwrap(), 16).unwrap();
		let a = c.at(4).map(|a| u8::from_str_radix(a, 16).unwrap()).unwrap_or(255);

		Ok((r, g, b, a))
	}
	else if let Some(c) = ::regex::Regex::new(r"^#([:xdigit:])([:xdigit:])([:xdigit:])([:xdigit:])?$").unwrap().captures(string) {
		let r = u8::from_str_radix(&iter::repeat(c.at(1).unwrap()).take(2).collect::<String>(), 16).unwrap();
		let g = u8::from_str_radix(&iter::repeat(c.at(2).unwrap()).take(2).collect::<String>(), 16).unwrap();
		let b = u8::from_str_radix(&iter::repeat(c.at(3).unwrap()).take(2).collect::<String>(), 16).unwrap();
		let a = c.at(4).map(|a| u8::from_str_radix(&iter::repeat(a).take(2).collect::<String>(), 16).unwrap()).unwrap_or(255);

		Ok((r, g, b, a))
	}
	else {
		Err("string is not a parsable color value")
	}
}
