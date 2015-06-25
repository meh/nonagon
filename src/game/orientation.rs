#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Orientation {
	pub roll:  f32,
	pub pitch: f32,
	pub yaw:   f32,
}

impl Orientation {
	pub fn new(roll: f32, pitch: f32, yaw: f32) -> Orientation {
		Orientation {
			roll:  roll,
			pitch: pitch,
			yaw:   yaw,
		}
	}
}
