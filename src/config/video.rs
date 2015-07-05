use docopt::ArgvMap;
use toml::{Table, Value, ParserError};

use super::error;

#[derive(Clone, Debug)]
pub struct Video {
	vsync:         bool,
	multisampling: Option<u16>,

	effects: Effects,
}

#[derive(Clone, Debug)]
pub struct Effects {
	bullet: Bullet,
}

#[derive(Clone, Debug)]
pub struct Bullet {
	glow: bool,
}

impl Default for Video {
	fn default() -> Video {
		Video {
			vsync:         true,
			multisampling: None,

			effects: Effects::default(),
		}
	}
}

impl Default for Effects {
	fn default() -> Effects {
		Effects {
			bullet: Bullet::default(),
		}
	}
}

impl Default for Bullet {
	fn default() -> Bullet {
		Bullet {
			glow: true,
		}
	}
}

impl Video {
	pub fn load(&mut self, args: &ArgvMap, toml: &Table) -> Result<(), ParserError> {
		if let Some(toml) = toml.get("video").and_then(|c| c.as_table()) {
			if let Some(value) = toml.get("vsync").and_then(|c| c.as_bool()) {
				self.vsync = value;
			}

			match toml.get("multisampling") {
				Some(&Value::Boolean(false)) =>
					self.multisampling = None,

				Some(&Value::Boolean(true)) =>
					self.multisampling = Some(2),

				Some(&Value::Integer(value)) =>
					self.multisampling = Some(value as u16),

				Some(_) =>
					return error("unknown `multisampling` value"),

				None =>
					()
			}

			if let Some(toml) = toml.get("effects").and_then(|c| c.as_table()) {
				if let Some(toml) = toml.get("bullet").and_then(|c| c.as_table()) {
					if let Some(value) = toml.get("glow").and_then(|c| c.as_bool()) {
						self.effects.bullet.glow = value;
					}
				}
			}
		}

		Ok(())
	}

	pub fn vsync(&self) -> bool {
		self.vsync
	}

	pub fn multisampling(&self) -> Option<u16> {
		self.multisampling
	}

	pub fn effects(&self) -> &Effects {
		&self.effects
	}
}

impl Effects {
	pub fn bullet(&self) -> &Bullet {
		&self.bullet
	}
}

impl Bullet {
	pub fn glow(&self) -> bool {
		self.glow
	}
}
