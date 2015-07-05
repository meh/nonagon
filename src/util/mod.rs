use std::path::{PathBuf};

#[macro_use]
mod macros;

mod color;
pub use self::color::{Color, Parse};

pub mod aspect;
pub use self::aspect::Aspect;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Fill {
	Color(Color),
	Texture(PathBuf),
}

impl<'a> From<&'a str> for Fill {
	fn from(string: &'a str) -> Fill {
		if let Ok(color) = color(string) {
			Fill::Color(color)
		}
		else {
			Fill::Texture(PathBuf::from(string))
		}
	}
}

pub fn deg(v: f32) -> f32 {
	::std::f32::consts::PI * v / 180.0
}

pub fn color<T: Parse>(value: T) -> Result<Color, &'static str> {
	Parse::parse(value)
}
