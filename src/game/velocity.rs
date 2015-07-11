#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
	pub z: f32,

	pub roll:  f32,
	pub pitch: f32,
	pub yaw:   f32,
}

impl Velocity {
	pub fn clear(&mut self) {
		self.x = 0.0;
		self.y = 0.0;
		self.z = 0.0;

		self.roll  = 0.0;
		self.pitch = 0.0;
		self.yaw   = 0.0;
	}
}
