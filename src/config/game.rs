use docopt::ArgvMap;

use toml::{Value, ParserError};

use game::ship::Shape;
use util::Fill;
use config::Load;

#[derive(Clone, Default, Debug)]
pub struct Game {
	ship: Ship,
}

impl Load for Game {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = toml.as_table().unwrap();

		if let Some(toml) = toml.get("game") {
			let toml = expect!(toml.as_table(), "`game` must be a table");

			if let Some(toml) = toml.get("ship") {
				try!(self.ship.load(args, toml));
			}
		}

		Ok(())
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

impl Game {
	pub fn ship(&self) -> &Ship {
		&self.ship
	}
}

impl Ship {
	pub fn shape(&self) -> Shape {
		self.shape
	}

	pub fn face(&self) -> Option<Fill> {
		self.face.clone()
	}

	pub fn border(&self) -> Option<Option<Fill>> {
		self.border.clone()
	}
}
