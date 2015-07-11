use util::{Fill, Aspect};
use super::{Position, Orientation, Velocity};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Shape {
	Sphere,
	Ray,
}

#[derive(Debug)]
pub struct Bullet {
	pub shape: Shape,
	pub fill:  Fill,

	pub position:    Position,
	pub orientation: Orientation,
	pub velocity:    Velocity,
}

impl Default for Bullet {
	fn default() -> Bullet {
		Bullet {
			shape: Shape::Sphere,
			fill:  Fill::from("#fff"),

			position:    Default::default(),
			orientation: Default::default(),
			velocity:    Default::default(),
		}
	}
}
