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
