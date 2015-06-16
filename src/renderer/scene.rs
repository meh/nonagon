use na::{self, Ortho3, Mat4, Vec3, Pnt3, Iso3, Rot3};

pub struct Scene {
	width:  u32,
	height: u32,

	projection: Mat4<f32>,
}

impl Scene {
	pub fn new(width: u32, height: u32) -> Scene {
		let projection = Ortho3::new(width as f32, height as f32, 0.1, 100.0);

		Scene {
			width:  width,
			height: height,

			projection: projection.to_mat(),
		}
	}

	pub fn is_vertical(&self) -> bool {
		self.width < self.height
	}

	pub fn is_horizontal(&self) -> bool {
		self.height < self.width
	}

	pub fn to_mat(&self) -> Mat4<f32> {
		self.projection
	}

	// TODO: actually use the given position
	pub fn position(&self, x: u16, y: u16) -> Mat4<f32> {
		if self.is_horizontal() {
			na::to_homogeneous(&Iso3::new(Vec3::new(x as f32, 0.0, -50.0), na::zero()))
		}
		else {
			na::to_homogeneous(&Iso3::new(Vec3::new(x as f32, 0.0, -50.0), na::zero()))
		}
	}

	pub fn scale(&self, factor: f32) -> Mat4<f32> {
		Mat4::new(factor,    0.0,    0.0, 0.0,
		             0.0, factor,    0.0, 0.0,
		             0.0,    0.0, factor, 0.0,
		             0.0,    0.0,    0.0, 1.0)
	}

	pub fn rotation(&self, roll: f32, pitch: f32, yaw: f32) -> Mat4<f32> {
		na::to_homogeneous(&Rot3::new_with_euler_angles(roll, pitch, yaw))
	}
}
