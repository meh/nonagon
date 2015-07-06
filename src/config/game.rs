use docopt::ArgvMap;
use toml::{Table, Value, ParserError};

use game::ship::Shape;
use util::Fill;

#[derive(Clone, Debug)]
pub struct Game {
	ship: Ship,
}

#[derive(Clone, Debug)]
pub struct Ship {
	shape:  Shape,
	face:   Option<Fill>,
	border: Option<Option<Fill>>,
}

impl Default for Game {
	fn default() -> Game {
		Game {
			ship: Ship::default(),
		}
	}
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

impl Game {
	pub fn load(&mut self, args: &ArgvMap, toml: &Table) -> Result<(), ParserError> {
		if let Some(toml) = toml.get("game") {
			let toml = expect!(toml.as_table(), "`game` must be a table");

			if let Some(toml) = toml.get("ship") {
				let toml = expect!(toml.as_table(), "`game.ship` must be a table");

				if let Some(value) = toml.get("shape") {
					let value = expect!(value.as_str(), "`game.ship.shape` must be a string");

					self.ship.shape = match value {
						"cube" =>
							Shape::Cube,

						_ =>
							expect!("`game.ship.shape` must be either 'cube' or ..."),
					}
				}

				if let Some(value) = toml.get("face") {
					let value = expect!(value.as_str(), "`game.ship.face` must be a string");

					self.ship.face = Some(Fill::from(value));
				}

				if let Some(value) = toml.get("border") {
					match value {
						&Value::String(ref value) =>
							self.ship.border = Some(Some(Fill::from(value))),

						&Value::Boolean(false) =>
							self.ship.border = Some(None),

						&Value::Boolean(true) =>
							(),

						_ =>
							expect!("`game.ship.border` must be a string or boolean"),
					}
				}
			}
		}

		Ok(())
	}

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
