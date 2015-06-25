use util::Color;
use super::{Position, Direction, Orientation};

pub enum Shape {
	Cube,
}

pub struct Ship {
	pub shape:       Shape,
	pub position:    Position,
	pub direction:   Direction,
	pub orientation: Orientation,
	pub color:       Color,
}
