use docopt::ArgvMap;

use toml::{Value, ParserError};

use config::Load;

#[derive(Clone, Debug)]
pub struct Analyzer {
	window: usize,

	threshold: Threshold,
}

impl Default for Analyzer {
	fn default() -> Self {
		Analyzer {
			window: 1024,

			threshold: Default::default(),
		}
	}
}

impl Load for Analyzer {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = toml.as_table().unwrap();

		if let Some(toml) = toml.get("analyzer") {
			let toml = expect!(toml.as_table(), "`analyzer` must be a table");

			if let Some(value) = toml.get("window") {
				self.window = expect!(value.as_integer(), "`analyzer.window` must be an integer") as usize;
			}

			if let Some(toml) = toml.get("threshold") {
				try!(self.threshold.load(args, toml));
			}
		}

		Ok(())
	}
}

impl Analyzer {
	pub fn window(&self) -> usize {
		self.window
	}

	pub fn threshold(&self) -> &Threshold {
		&self.threshold
	}
}

#[derive(Clone, Debug)]
pub struct Threshold {
	size:        usize,
	sensitivity: f64,
}

impl Default for Threshold {
	fn default() -> Self {
		Threshold {
			size:        20,
			sensitivity: 1.5,
		}
	}
}

impl Load for Threshold {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`analyzer.threshold` must be a table");

		if let Some(value) = toml.get("size") {
			self.size = expect!(value.as_integer(), "`analyzer.threshold.size` must be an integer") as usize;
		}

		if let Some(value) = toml.get("sensitivity") {
			self.sensitivity = expect!(value.as_float(), "`analyzer.threshold.sensitivity` must be a float") as f64;
		}

		Ok(())
	}
}

impl Threshold {
	pub fn size(&self) -> usize {
		self.size
	}

	pub fn sensitivity(&self) -> f64 {
		self.sensitivity
	}
}
