use na::{self, Ortho3, Mat4, Vec3, Iso3, Rot3};
use ffmpeg::Rational;

use game::{Orientation, Position, Aspect};
use util::deg;

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

	pub fn to_mat(&self) -> Mat4<f32> {
		self.projection
	}

	pub fn position(&self, Position(x, y): Position) -> Mat4<f32> {
		let x = x as f32 * self.width as f32 / self.aspect.width().unwrap() as f32;
		let y = y as f32 * self.height as f32 / self.aspect.height().unwrap() as f32;

		na::to_homogeneous(&Iso3::new(Vec3::new(
			if x > self.width as f32 / 2.0 {
				-((self.width as f32 / 2.0) - x)
			}
			else {
				x - self.width as f32 / 2.0
			},

			-if y > self.height as f32 / 2.0 {
				-((self.height as f32 / 2.0) - y)
			}
			else {
				y - self.height as f32 / 2.0
			},

			-50.0), na::zero()))
	}

	pub fn orientation(&self, orientation: Orientation) -> Mat4<f32> {
		self.rotation(deg(orientation.roll), deg(orientation.pitch), deg(orientation.yaw))
	}

	pub fn scale(&self, factor: f32) -> Mat4<f32> {
		let factor = (factor * self.width as f32 * self.height as f32) /
			(self.aspect.width().unwrap() as f32 * self.aspect.height().unwrap() as f32);

		Mat4::new(factor,    0.0,    0.0, 0.0,
		             0.0, factor,    0.0, 0.0,
		             0.0,    0.0, factor, 0.0,
		             0.0,    0.0,    0.0, 1.0)
	}

	pub fn rotation(&self, roll: f32, pitch: f32, yaw: f32) -> Mat4<f32> {
		na::to_homogeneous(&Rot3::new_with_euler_angles(roll, pitch, yaw))
	}
}
