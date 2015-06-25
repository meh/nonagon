#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Direction(pub i16, pub i16);

impl Direction {
	pub fn x(&self) -> i16 {
		self.0
	}

	pub fn y(&self) -> i16 {
		self.1
	}
}
