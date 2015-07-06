use docopt::ArgvMap;
use toml::{Table, ParserError};

#[derive(Clone, Debug)]
pub struct Audio {
	mute: bool,
	only: bool,
}

impl Default for Audio {
	fn default() -> Audio {
		Audio {
			mute: false,
			only: false,
		}
	}
}

impl Audio {
	pub fn load(&mut self, args: &ArgvMap, toml: &Table) -> Result<(), ParserError> {
		if let Some(toml) = toml.get("audio") {
			let toml = expect!(toml.as_table(), "`audio` must be a table");

			if let Some(value) = toml.get("only") {
				self.only = expect!(value.as_bool(), "`audio.only` must be a boolean");
			}

			if let Some(value) = toml.get("mute") {
				self.mute = expect!(value.as_bool(), "`audio.mute` must be a boolean");
			}
		}

		if args.get_bool("--audio-only") {
			self.only = true;
		}

		if args.get_bool("--mute") {
			self.mute = true;
		}

		Ok(())
	}

	pub fn mute(&self) -> bool {
		self.mute
	}

	pub fn only(&self) -> bool {
		self.only
	}
}
