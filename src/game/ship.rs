use util::{rgba, Color};
use super::Position;

pub enum Shape {
	Cube,
}

pub struct Ship {
	pub shape:    Shape,
	pub position: Position,
	pub color:    Color,
}

impl Ship {
	pub fn cube() -> Self {
		Ship {
			shape:    Shape::Cube,
			position: Position::new(0, 0),
			color:    rgba(255, 255, 255, 255),
		}
	}
}
