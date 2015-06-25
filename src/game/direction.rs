#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Direction {
	pub x: i16,
	pub y: i16,
}

impl Direction {
	pub fn new(x: i16, y: i16) -> Direction {
		Direction {
			x: x,
			y: y,
		}
	}
}
