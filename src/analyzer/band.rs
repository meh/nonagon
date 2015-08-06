use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Band {
	name: Option<String>,
	low:  u32,
	high: u32,
}

impl Band {
	pub fn new<T: Into<String>>(name: Option<T>, low: u32, high: u32) -> Self {
		Band {
			name: name.map(|v| v.into()),
			low:  low,
			high: high,
		}
	}

	#[inline(always)]
	pub fn name(&self) -> Option<&str> {
		self.name.as_ref().map(|n| n.as_ref())
	}

	pub fn low(&self) -> u32 {
		self.low
	}

	pub fn high(&self) -> u32 {
		self.high
	}
}

impl From<Range<u32>> for Band {
	fn from(value: Range<u32>) -> Self {
		Band::new::<&str>(None, value.start, value.end)
	}
}

impl Into<Range<u32>> for Band {
	fn into(self) -> Range<u32> {
		self.low() .. self.high() + 1
	}
}

impl PartialEq for Band {
	fn eq(&self, other: &Band) -> bool {
		self.low() == other.low() && self.high() == other.high()
	}
}

impl Eq for Band { }
