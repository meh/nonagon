use std::ops::{Deref, DerefMut};

use na::Vec3;

pub enum Ship {
	Cube(Details),
}

pub struct Details {
	position: Vec3<f32>,
}

impl Details {
	pub fn new() -> Self {
		Details {
			position: Vec3::new(0.0, 0.0, 0.0),
		}
	}

	pub fn position(&self) -> &Vec3<f32> {
		&self.position
	}

	pub fn update_position<F: Fn(Vec3<f32>) -> Vec3<f32>>(mut self, f: F) -> Self {
		self.position = f(self.position);

		self
	}
	
	pub fn position_mut(&mut self) -> &mut Vec3<f32> {
		&mut self.position
	}
}

impl Ship {
	pub fn cube() -> Self {
		Ship::Cube(Details::new())
	}
}

impl Deref for Ship {
	type Target = Details;

	fn deref(&self) -> &<Self as Deref>::Target {
		match self {
			&Ship::Cube(ref details) =>
				details
		}
	}
}

impl DerefMut for Ship {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		match self {
			&mut Ship::Cube(ref mut details) =>
				details
		}
	}
}
