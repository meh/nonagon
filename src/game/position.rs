use util::Aspect;

#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub struct Position {
	// 0 < x < aspect.width
	pub x: f32,

	// 0 < y < aspect.height
	pub y: f32,

	// -100.0 < z < 100.0
	pub z: f32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Axis {
	X(f32),
	Y(f32),
	Z(f32),
}

#[derive(Clone, Copy, Debug)]
pub enum ValidationError {
	OutOfBounds(Axis),
}

impl Position {
	pub fn validate(self, aspect: &Aspect) -> Result<Self, ValidationError> {
		if self.x < 0.0 || self.x > aspect.width() as f32 {
			return Err(ValidationError::OutOfBounds(Axis::X(self.x)));
		}

		if self.y < 0.0 || self.y > aspect.height() as f32 {
			return Err(ValidationError::OutOfBounds(Axis::Y(self.y)));
		}

		if self.z < -100.0 || self.z > 100.0 {
			return Err(ValidationError::OutOfBounds(Axis::Z(self.z)));
		}

		Ok(self)
	}
}
