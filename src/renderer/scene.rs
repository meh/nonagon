use na::{self, Ortho3, Mat4, Vec3, Iso3, Rot3};
use ffmpeg::Rational;

use game::{Orientation, Position};
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

	pub fn is_vertical(&self) -> bool {
		self.width < self.height
	}

	pub fn is_horizontal(&self) -> bool {
		self.height < self.width
	}

	pub fn to_mat(&self) -> Mat4<f32> {
		self.projection
	}

	pub fn position(&self, Position(x, y): Position) -> Mat4<f32> {
		if self.is_horizontal() {
			unimplemented!();
		}
		else {
			// adapt the values to the standard 3:4 viewport
			let x = x as f32 * self.width as f32 / 480.0;
			let y = y as f32 * self.height as f32 / 640.0;

			na::to_homogeneous(&Iso3::new(Vec3::new(
				// if x is beyond half screen
				if x > self.width as f32 / 2.0 {
					// make it go from 0 to 240
					-((self.width as f32 / 2.0) - x)
				}
				else {
					// make it go from -240 to 0
					x - self.width as f32 / 2.0
				},

				// if y is beyond half screen
				-if y > self.height as f32 / 2.0 {
					// make it go from 0 to to 320
					-((self.height as f32 / 2.0) - y)
				}
				else {
					// make it go from -320 to 0
					y - self.height as f32 / 2.0
				},

				// middle z because we're orthogonally projecting anyway
				-50.0), na::zero()))
		}
	}

	pub fn orientation(&self, orientation: Orientation) -> Mat4<f32> {
		self.rotation(deg(orientation.roll), deg(orientation.pitch), deg(orientation.yaw))
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
