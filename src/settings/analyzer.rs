use std::ops::Range;

use docopt::ArgvMap;

use toml::{Value, ParserError};

use settings::Load;
use util::Color;

#[derive(Clone, Default, Debug)]
pub struct Analyzer {
	window:   Window,
	beat:     Beat,
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
	size:   usize,
	hop:    usize,
	filter: Filter,
}

impl Default for Window {
	fn default() -> Self {
		Window {
			size:   1024,
			hop:    512,
			filter: Filter::None,
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

		if let Some(value) = top.get("filter") {
			match value {
				&Value::String(ref filter) =>
					self.filter = Filter::from(filter),

				&Value::Boolean(false) =>
					self.filter = Filter::None,

				_ =>
					expect!("`analyzer.window.filter` must be either \"hamming\" or false"),
			}
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
	pub fn filter(&self) -> Filter {
		self.filter
	}
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Filter {
	None,
	Hamming,
}

impl<T: AsRef<str>> From<T> for Filter {
	fn from(value: T) -> Filter {
		match value.as_ref() {
			"hamming" =>
				Filter::Hamming,

			_ =>
				panic!("unsupported filter"),
		}
	}
}

#[derive(Clone, Debug)]
pub struct Beat {
	throttle:  f64,
	threshold: Threshold,

	bands: Vec<Band>,
}

impl Default for Beat {
	fn default() -> Self {
		Beat {
			throttle:  0.0,
			threshold: Default::default(),

			bands: Vec::new(),
		}
	}
}

impl Load for Beat {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let top = expect!(toml.as_table(), "`analyzer.beat` must be a table");

		if let Some(value) = top.get("throttle") {
			self.throttle = expect!(value.as_float(), "`analyzer.beat.throttle` must be a float");
		}

		if let Some(toml) = top.get("threshold") {
			try!(self.threshold.load(args, toml));
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
	pub fn throttle(&self) -> f64 {
		self.throttle
	}

	#[inline(always)]
	pub fn threshold(&self) -> &Threshold {
		&self.threshold
	}

	#[inline(always)]
	pub fn bands(&self) -> &[Band] {
		&*self.bands
	}
}

#[derive(Clone, Debug)]
pub struct Band {
	name:  Option<String>,
	color: Option<Color>,

	range:     Range<u32>,
	threshold: Threshold,
	throttle:  f64,
}

impl Default for Band {
	fn default() -> Self {
		Band {
			name:      None,
			color:     None,
			range:     Range { start: 0, end: 0 },
			threshold: Default::default(),
			throttle:  0.0,
		}
	}
}

impl Load for Band {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let top = expect!(toml.as_table(), "`analyzer.beat.band.*` must be a table");

		if let Some(value) = top.get("name") {
			self.name = Some(expect!(value.as_str(),
				"`analyzer.beat.band.*.name` must be a string").to_owned());
		}

		if let Some(value) = top.get("color") {
			self.color = Some(Color::from(expect!(value.as_str(),
				"`analyzer.beat.band.*.color` must be a string")));
		}

		if let Some(value) = top.get("range") {
			match value {
				&Value::Array(ref range) => {
					if range.len() != 2 {
						expect!("`analyzer.beat.band.*.range` must be an array of two elements");
					}

					let lo = expect!(range[0].as_integer(),
						"`analyzer.beat.band.*.range.0` must be an integer");

					let hi = expect!(range[1].as_integer(),
						"`analyzer.beat.band.*.range.1` must be an integer");

					self.range = lo as u32 .. hi as u32;
				},

				&Value::Boolean(false) =>
					(),

				_ =>
					expect!("`analyzer.beat.band.*.range` must be an array or false")
			}
		}

		if let Some(value) = top.get("throttle") {
			self.throttle = expect!(value.as_float(), "`analyzer.beat.band.*.throttle` must be a float");
		}

		try!(self.threshold.load(args, &Value::Table(top.clone())));

		Ok(())
	}
}

impl Band {
	#[inline(always)]
	pub fn name(&self) -> Option<&str> {
		self.name.as_ref().map(|n| n.as_ref())
	}

	#[inline(always)]
	pub fn color(&self) -> Option<Color> {
		self.color
	}

	#[inline(always)]
	pub fn range(&self) -> &Range<u32> {
		&self.range
	}

	#[inline(always)]
	pub fn threshold(&self) -> &Threshold {
		&self.threshold
	}

	#[inline(always)]
	pub fn throttle(&self) -> f64 {
		self.throttle
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
			size:        25,
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
