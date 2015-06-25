use util::Color;
use super::{Position, Direction};

pub enum Shape {
	Cube,
}

pub struct Ship {
	pub shape:     Shape,
	pub position:  Position,
	pub direction: Direction,
	pub color:     Color,
}
