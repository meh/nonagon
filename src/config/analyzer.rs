use docopt::ArgvMap;

use toml::{Value, ParserError};

use config::Load;
use analyzer::Range;

#[derive(Clone, Default, Debug)]
pub struct Analyzer {
	window: Window,
	beat:   Beat,
}

impl Load for Analyzer {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let top = toml.as_table().unwrap();

		if let Some(toml) = top.get("analyzer") {
			let toml = expect!(toml.as_table(), "`analyzer` must be a table");

			if let Some(toml) = toml.get("window") {
				try!(self.window.load(args, toml));
			}

			if let Some(toml) = toml.get("beat") {
				try!(self.beat.load(args, toml));
			}
		}

		Ok(())
	}
}

impl Analyzer {
	#[inline(always)]
	pub fn window(&self) -> &Window {
		&self.window
	}

	#[inline(always)]
	pub fn beat(&self) -> &Beat {
		&self.beat
	}

	pub fn min_cache(&self) -> f64 {
		let mut result = (1.0 / 44100.0) * (self.beat().threshold().size() * 2 + 1) as f64;

		for band in self.beat().bands() {
			let current = (1.0 / 44100.0) * (band.threshold().size() * 2 + 1) as f64;

			if current > result {
				result = current;
			}
		}

		result
	}
}

#[derive(Clone, Debug)]
pub struct Window {
	size:    usize,
	hop:     usize,
	hamming: bool,
}

impl Default for Window {
	fn default() -> Self {
		Window {
			size:    1024,
			hop:     512,
			hamming: true,
		}
	}
}

impl Load for Window {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let top = expect!(toml.as_table(), "`analyzer.window` must be a table");

		if let Some(value) = top.get("size") {
			self.size = expect!(value.as_integer(), "`analyzer.window.size` must be an integer") as usize;
		}

		if let Some(value) = top.get("hop") {
			self.hop = expect!(value.as_integer(), "`analyzer.window.hop` must be an integer") as usize;
		}

		if let Some(value) = top.get("hamming") {
			self.hamming = expect!(value.as_bool(), "`analyzer.window.hamming` must be a boolean");
		}

		if self.size < self.hop {
			expect!("`analyzer.window.hop` must be lesser than or equal to `analyzer.window.size`");
		}

		Ok(())
	}
}

impl Window {
	#[inline(always)]
	pub fn size(&self) -> usize {
		self.size
	}

	#[inline(always)]
	pub fn hop(&self) -> usize {
		self.hop
	}

	#[inline(always)]
	pub fn is_hamming(&self) -> bool {
		self.hamming
	}
}

#[derive(Clone, Debug)]
pub struct Beat {
	threshold: Threshold,

	ignore: bool,
	bands:  Vec<Band>,
}

impl Default for Beat {
	fn default() -> Self {
		Beat {
			ignore:    true,
			threshold: Default::default(),
			bands:     Vec::new(),
		}
	}
}

impl Load for Beat {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let top = expect!(toml.as_table(), "`analyzer.beat` must be a table");

		if let Some(toml) = top.get("threshold") {
			try!(self.threshold.load(args, toml));
		}

		if let Some(value) = top.get("ignore-missing") {
			self.ignore = expect!(value.as_bool(), "`analyzer.beat.ignore-missing` must be a boolean");
		}

		if let Some(toml) = top.get("band") {
			let  toml = expect!(toml.as_slice(), "`analyzer.band` must be an array");
			self.bands = vec![Default::default(); toml.len()];

			for (band, value) in self.bands.iter_mut().zip(toml.iter()) {
				if let Some(toml) = top.get("threshold") {
					try!(band.threshold.load(args, toml));
				}

				try!(band.load(args, value));
			}
		}

		Ok(())
	}
}

impl Beat {
	#[inline(always)]
	pub fn threshold(&self) -> &Threshold {
		&self.threshold
	}

	#[inline(always)]
	pub fn ignore_missing(&self) -> bool {
		self.ignore
	}

	#[inline(always)]
	pub fn bands(&self) -> &[Band] {
		&*self.bands
	}
}

#[derive(Clone, Debug)]
pub struct Band {
	range:     Range,
	threshold: Threshold,
}

impl Default for Band {
	fn default() -> Self {
		Band {
			range:     Range::default(),
			threshold: Default::default(),
		}
	}
}

impl Load for Band {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let top = expect!(toml.as_table(), "`analyzer.beat.band.*` must be a table");

		if let Some(value) = top.get("range") {
			match value {
				&Value::Array(ref range) => {
					if range.len() != 2 {
						expect!("`analyzer.beat.band.*.range` must be an array of two elements");
					}

					let lo = expect!(range[0].as_integer(), "`analyzer.beat.band.*.range.0` must be an integer");
					let hi = expect!(range[1].as_integer(), "`analyzer.beat.band.*.range.1` must be an integer");

					self.range = Range::new(lo as u32, hi as u32);
				},

				&Value::Boolean(false) =>
					(),

				_ =>
					expect!("`analyzer.beat.band.*.range` must be an array or false")
			}
		}

		try!(self.threshold.load(args, &Value::Table(top.clone())));

		Ok(())
	}
}

impl Band {
	#[inline(always)]
	pub fn range(&self) -> Range {
		self.range.clone()
	}

	#[inline(always)]
	pub fn threshold(&self) -> &Threshold {
		&self.threshold
	}
}

#[derive(Clone, Debug)]
pub struct Threshold {
	band:        Option<Range>,
	size:        usize,
	sensitivity: f64,
}

impl Default for Threshold {
	fn default() -> Self {
		Threshold {
			band:        None,
			size:        20,
			sensitivity: 1.5,
		}
	}
}

impl Load for Threshold {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let top = expect!(toml.as_table(), "`analyzer.threshold` must be a table");

		if let Some(value) = top.get("size") {
			self.size = expect!(value.as_integer(), "`analyzer.beat.threshold.size` must be an integer") as usize;
		}

		if let Some(value) = top.get("sensitivity") {
			self.sensitivity = expect!(value.as_float(), "`analyzer.beat.threshold.sensitivity` must be a float") as f64;
		}

		Ok(())
	}
}

impl Threshold {
	#[inline(always)]
	pub fn size(&self) -> usize {
		self.size
	}

	#[inline(always)]
	pub fn sensitivity(&self) -> f64 {
		self.sensitivity
	}
}
