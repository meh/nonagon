use docopt::ArgvMap;
use toml::{Table, ParserError};

use game::ship::Shape;
use super::error;
use util::Fill;

#[derive(Clone, Debug)]
pub struct Game {
	ship: Ship,
}

#[derive(Clone, Debug)]
pub struct Ship {
	shape:  Shape,
	face:   Option<Fill>,
	border: Option<Fill>,
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
		if let Some(toml) = toml.get("game").and_then(|c| c.as_table()) {
			if let Some(toml) = toml.get("ship").and_then(|c| c.as_table()) {
				if let Some(value) = toml.get("shape").and_then(|c| c.as_str()) {
					self.ship.shape = match value {
						"cube" =>
							Shape::Cube,

						_ =>
							return error("unknown shape")
					}
				}

				if let Some(value) = toml.get("face").and_then(|c| c.as_str()) {
					self.ship.face = Some(Fill::from(value));
				}

				if let Some(value) = toml.get("border").and_then(|c| c.as_str()) {
					self.ship.border = Some(Fill::from(value));
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

	pub fn border(&self) -> Option<Fill> {
		self.border.clone()
	}
}
