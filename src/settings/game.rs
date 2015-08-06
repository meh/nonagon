use std::collections::HashMap;

use docopt::ArgvMap;
use toml::{Value, ParserError};
use ffmpeg::Rational;
use regex::Regex;

use game::ship::Shape;
use util::Fill;
use settings::Load;

#[derive(Clone, Debug)]
pub struct Game {
	step: f64,

	window: Window,
	ship:   Ship,
}

impl Default for Game {
	fn default() -> Self {
		Game {
			step: 0.015,

			window: Window::default(),
			ship:   Ship::default(),
		}
	}
}

impl Load for Game {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = toml.as_table().unwrap();

		if let Some(toml) = toml.get("game") {
			let toml = expect!(toml.as_table(), "`game` must be a table");

			if let Some(value) = toml.get("step") {
				self.step = expect!(value.as_float(), "`game.step` must be a float");
			}

			if let Some(toml) = toml.get("window") {
				try!(self.window.load(args, toml));
			}

			if let Some(toml) = toml.get("ship") {
				try!(self.ship.load(args, toml));
			}
		}

		Ok(())
	}
}

impl Game {
	#[inline(always)]
	pub fn step(&self) -> f64 {
		self.step
	}

	#[inline(always)]
	pub fn window(&self) -> &Window {
		&self.window
	}

	#[inline(always)]
	pub fn ship(&self) -> &Ship {
		&self.ship
	}
}

#[derive(Clone, Debug)]
pub struct Window {
	aspects: HashMap<String, Window>,

	width:  Option<u32>,
	height: Option<u32>,
}

impl Default for Window {
	fn default() -> Self {
		Window {
			aspects: HashMap::new(),

			width:  None,
			height: None,
		}
	}
}

impl Load for Window {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`game.window` must be a table");

		if let Some(value) = toml.get("width") {
			self.width = Some(expect!(value.as_integer(), "`game.window.width` must be an integer") as u32);
		}

		if let Some(value) = toml.get("height") {
			self.height = Some(expect!(value.as_integer(), "`game.window.height` must be an integer") as u32);
		}

		for (key, toml) in toml {
			if Regex::new(r"(\d+)-(\d+)").unwrap().is_match(key) {
				let mut window = Window::default();
				try!(window.load(args, toml));

				self.aspects.insert(key.clone(), window);
			}
		}

		Ok(())
	}
}

impl Window {
	#[inline(always)]
	pub fn aspect(&self, aspect: Rational) -> Option<&Window> {
		self.aspects.get(&format!("{}-{}", aspect.numerator(), aspect.denominator()))
	}

	#[inline(always)]
	pub fn width(&self) -> Option<u32> {
		self.width
	}

	#[inline(always)]
	pub fn height(&self) -> Option<u32> {
		self.height
	}
}

#[derive(Clone, Debug)]
pub struct Ship {
	shape:  Shape,
	face:   Option<Fill>,
	border: Option<Option<Fill>>,
}

impl Default for Ship {
	fn default() -> Ship {
		Ship {
			shape:  Shape::Cube,
			face:   None,
			border: None,
		}
	}
}

impl Load for Ship {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`game.ship` must be a table");

		if let Some(value) = toml.get("shape") {
			let value = expect!(value.as_str(), "`game.ship.shape` must be a string");

			self.shape = match value {
				"cube" =>
					Shape::Cube,
				
				"tetrahedron" =>
					Shape::Tetrahedron,

				"octahedron" =>
					Shape::Octahedron,

				_ =>
					expect!("`game.ship.shape` must be 'cube' or 'tetrahedron' or 'octahedron'"),
			}
		}

		if let Some(value) = toml.get("face") {
			let value = expect!(value.as_str(), "`game.ship.face` must be a string");

			self.face = Some(Fill::from(value));
		}

		if let Some(value) = toml.get("border") {
			match value {
				&Value::String(ref value) =>
					self.border = Some(Some(Fill::from(value))),

				&Value::Boolean(false) =>
					self.border = Some(None),

				&Value::Boolean(true) =>
					(),

				_ =>
					expect!("`game.ship.border` must be a string or boolean"),
			}
		}

		Ok(())
	}
}

impl Ship {
	#[inline(always)]
	pub fn shape(&self) -> Shape {
		self.shape
	}

	#[inline(always)]
	pub fn face(&self) -> Option<Fill> {
		self.face.clone()
	}

	#[inline(always)]
	pub fn border(&self) -> Option<Option<Fill>> {
		self.border.clone()
	}
}
