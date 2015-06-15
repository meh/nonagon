#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
	pub x: u16,
	pub y: u16,
}

impl Position {
	pub fn new(x: u16, y: u16) -> Position {
		Position {
			x: x,
			y: y,
		}
	}
}
