#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position(pub u16, pub u16);

impl Position {
	pub fn x(&self) -> u16 {
		self.0
	}

	pub fn y(&self) -> u16 {
		self.1
	}
}
