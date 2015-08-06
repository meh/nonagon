use docopt::ArgvMap;

use toml::{Value, ParserError};

use settings::Load;

#[derive(Clone, Debug)]
pub struct Audio {
	music: bool,
	only:  bool,
}

impl Default for Audio {
	fn default() -> Audio {
		Audio {
			music: true,
			only:  false,
		}
	}
}

impl Load for Audio {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = toml.as_table().unwrap();

		if let Some(toml) = toml.get("audio") {
			let toml = expect!(toml.as_table(), "`audio` must be a table");

			if let Some(value) = toml.get("only") {
				self.only = expect!(value.as_bool(), "`audio.only` must be a boolean");
			}

			if let Some(value) = toml.get("music") {
				self.music = expect!(value.as_bool(), "`audio.music` must be a boolean");
			}
		}

		if args.get_bool("--audio-only") {
			self.only = true;
		}

		if args.get_bool("--no-music") {
			self.music = false;
		}

		Ok(())
	}
}

impl Audio {
	#[inline(always)]
	pub fn music(&self) -> bool {
		self.music
	}

	#[inline(always)]
	pub fn only(&self) -> bool {
		self.only
	}
}
