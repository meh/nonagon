#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
	pub z: f32,

	pub roll:  f32,
	pub pitch: f32,
	pub yaw:   f32,
}
