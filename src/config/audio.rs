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
		if let Some(toml) = toml.get("audio").and_then(|c| c.as_table()) {
			if let Some(value) = toml.get("only").and_then(|c| c.as_bool()) {
				self.only = value;
			}

			if let Some(value) = toml.get("mute").and_then(|c| c.as_bool()) {
				self.mute = value;
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
