use util::{Fill, Aspect};
use super::{Update, Position, Orientation, Velocity};

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

impl Update for Ship {
	fn update(&mut self, aspect: &Aspect) {
		#[inline(always)]
		fn up(value: f32, velocity: f32, min: f32, max: f32, around: bool) -> f32 {
			let new = value + velocity;

			if new + velocity < min {
				if around {
					max
				}
				else {
					min
				}
			}
			else if new > max {
				if around {
					min
				}
				else {
					max
				}
			}
			else {
				new
			}
		}

		self.position.x = up(self.position.x, self.velocity.x,    0.0, aspect.width() as f32,  false);
		self.position.y = up(self.position.y, self.velocity.y,    0.0, aspect.height() as f32, false);
		self.position.z = up(self.position.z, self.velocity.z, -100.0, 100.0,                  false);

		self.orientation.roll  = up(self.orientation.roll,  self.velocity.roll,  0.0, 360.0, true);
		self.orientation.pitch = up(self.orientation.pitch, self.velocity.pitch, 0.0, 360.0, true);
		self.orientation.yaw   = up(self.orientation.yaw,   self.velocity.yaw,   0.0, 360.0, true);
	}
}
