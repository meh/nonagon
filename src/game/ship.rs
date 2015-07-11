use util::{Fill, Aspect};
use super::{Position, Orientation, Velocity};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Shape {
	Cube,
	Tetrahedron,
	Octahedron,
}

#[derive(Debug)]
pub struct Ship {
	pub shape:  Shape,
	pub face:   Fill,
	pub border: Option<Fill>,

	pub position:    Position,
	pub orientation: Orientation,
	pub velocity:    Velocity,
	pub scale:       f32,
}

impl Default for Ship {
	fn default() -> Ship {
		Ship {
			shape:  Shape::Cube,
			face:   Fill::from("#fff"),
			border: Some(Fill::from("#000")),

			position:    Default::default(),
			orientation: Default::default(),
			velocity:    Default::default(),
			scale:       1.0,
		}
	}
}
