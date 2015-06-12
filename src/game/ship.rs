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
}

impl Ship {
	pub fn cube() -> Self {
		Ship::Cube(Details::new())
	}
}
