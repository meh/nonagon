pub mod color;
pub use self::color::Color;

#[macro_use]
pub mod macros;

pub fn deg(v: f32) -> f32 {
	::std::f32::consts::PI * v / 180.0
}

pub fn color<T: Into<Color>>(value: T) -> Color {
	value.into()
}
