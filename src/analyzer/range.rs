#[derive(Eq, PartialEq, Copy, Clone, Default, Debug)]
pub struct Range {
	pub low:  u32,
	pub high: u32,
}

impl Range {
	pub fn new(low: u32, high: u32) -> Range {
		Range {
			low:  low,
			high: high,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.low == 0 && self.high == 0
	}
}
