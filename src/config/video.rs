use docopt::ArgvMap;
use toml::{Table, Value, ParserError};

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
		if let Some(toml) = toml.get("video") {
			let toml = expect!(toml.as_table(), "`video` must be a table");

			if let Some(value) = toml.get("vsync") {
				self.vsync = expect!(value.as_bool(), "`video.vsync` must be boolean");
			}

			if let Some(value) = toml.get("multisampling") {
				match value {
					&Value::Boolean(false) =>
						self.multisampling = None,

					&Value::Boolean(true) =>
						self.multisampling = Some(2),

					&Value::Integer(value) =>
						self.multisampling = Some(value as u16),

					_ =>
						expect!("`video.multisampling` must be a boolean or integer"),
				}
			}

			if let Some(toml) = toml.get("effects") {
				let toml = expect!(toml.as_table(), "`video.effects` must be a table");

				if let Some(toml) = toml.get("bullet") {
					let toml = expect!(toml.as_table(), "`video.effects.bullet` must be a table");

					if let Some(value) = toml.get("glow") {
						self.effects.bullet.glow = expect!(value.as_bool(), "`video.effects.bullet.glow` must be a boolean");
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
