use util::Color;
use super::{Position, Direction};

pub enum Shape {
	Cube,
	Ray,
}

pub struct Bullet {
	pub shape:     Shape,
	pub position:  Position,
	pub direction: Direction,
	pub color:     Color,
}
