use util::{Fill, Aspect};
use super::{Position, Orientation, Velocity};

#[derive(Debug)]
pub enum Bullet {
	Sphere {
		fill: Fill,

		position:    Position,
		orientation: Orientation,
		velocity:    Velocity,
	},

	Ray {
		fill:  Fill,
		width: f32,

		position:    Position,
		orientation: Orientation,
		velocity:    Velocity,
	}
}

impl Default for Bullet {
	fn default() -> Bullet {
		Bullet::Sphere {
			fill: Fill::from("#fff"),

			position:    Default::default(),
			orientation: Default::default(),
			velocity:    Default::default(),
		}
	}
}
