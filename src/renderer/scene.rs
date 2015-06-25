use na::{self, Ortho3, Mat4, Vec3, Iso3, Rot3};
use ffmpeg::Rational;

pub struct Scene {
	width:  u32,
	height: u32,
	aspect: Rational,

	projection: Mat4<f32>,
}

impl Scene {
	pub fn new(width: u32, height: u32, aspect: Rational) -> Scene {
		let projection = Ortho3::new(width as f32, height as f32, 0.1, 100.0);

		Scene {
			width:  width,
			height: height,
			aspect: aspect.reduce(),

			projection: projection.to_mat(),
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn aspect(&self) -> Rational {
		self.aspect
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

	pub fn position(&self, x: u16, y: u16) -> Mat4<f32> {
		if self.is_horizontal() {
			unimplemented!();
		}
		else {
			let mut x = x as f32 * self.width as f32 / 480.0;
			let mut y = y as f32 * self.height as f32 / 640.0;

			if x > self.width as f32 / 2.0 {
				x = x / 2.0;
			}
			else {
				x = x - self.width as f32 / 2.0;
			}

			if y > self.height as f32 / 2.0 {
				y = y / 2.0;
			}
			else {
				y = y - self.height as f32 / 2.0;
			}

			na::to_homogeneous(&Iso3::new(Vec3::new(x, -y, -50.0), na::zero()))
		}
	}

	pub fn scale(&self, mut factor: f32) -> Mat4<f32> {
		if self.is_horizontal() {
			unimplemented!();
		}
		else {
			factor = (factor * self.width as f32 * self.height as f32) / (640.0 * 480.0);
		}

		Mat4::new(factor,    0.0,    0.0, 0.0,
		             0.0, factor,    0.0, 0.0,
		             0.0,    0.0, factor, 0.0,
		             0.0,    0.0,    0.0, 1.0)
	}

	pub fn rotation(&self, roll: f32, pitch: f32, yaw: f32) -> Mat4<f32> {
		na::to_homogeneous(&Rot3::new_with_euler_angles(roll, pitch, yaw))
	}
}
