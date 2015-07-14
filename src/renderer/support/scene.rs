use std::cmp;

use na::{self, Ortho3, Mat4, Vec3, Iso3, Rot3};
use ffmpeg::Rational;

use game::{Orientation, Position};
use util::{deg, Aspect};

pub struct Scene {
	width:  u32,
	height: u32,
	aspect: Rational,

	projection: Mat4<f32>,
}

impl Scene {
	pub fn new(aspect: Rational) -> Scene {
		Scene {
			width:  0,
			height: 0,
			aspect: aspect.reduce(),

			projection: na::zero(),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.width      = width;
		self.height     = height;
		self.projection = Ortho3::new(width as f32, height as f32, 0.1, 1000.0).to_mat();
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

	pub fn position(&self, Position { x, y, .. }: Position) -> Mat4<f32> {
		let (x, y) = if self.aspect.is_vertical() {
			(x * self.width as f32 / self.aspect.width() as f32,
			 y * self.height as f32 / self.aspect.height() as f32)
		}
		else {
			((self.aspect.height() as f32 - y) * self.width as f32 / self.aspect.height() as f32,
			 x * self.height as f32 / self.aspect.width() as f32)
		};

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

			-500.0), na::zero()))
	}

	pub fn orientation(&self, orientation: Orientation) -> Mat4<f32> {
		self.rotation(deg(orientation.roll), deg(orientation.pitch), deg(orientation.yaw))
	}

	pub fn scale(&self, factor: f32) -> Mat4<f32> {
		let factor = (factor * cmp::max(self.width, self.height) as f32) /
			cmp::max(self.aspect.width(), self.aspect.height()) as f32;

		Mat4::new(factor,    0.0,    0.0, 0.0,
		             0.0, factor,    0.0, 0.0,
		             0.0,    0.0, factor, 0.0,
		             0.0,    0.0,    0.0, 1.0)
	}

	pub fn transform(&self, x: f32, y: f32, z: f32) -> Mat4<f32> {
		let x = (x * cmp::max(self.width, self.height) as f32) /
			cmp::max(self.aspect.width(), self.aspect.height()) as f32;

		let y = (y * cmp::max(self.width, self.height) as f32) /
			cmp::max(self.aspect.width(), self.aspect.height()) as f32;

		let z = (z * cmp::max(self.width, self.height) as f32) /
			cmp::max(self.aspect.width(), self.aspect.height()) as f32;

		Mat4::new(  x, 0.0, 0.0, 0.0,
		          0.0,   y, 0.0, 0.0,
		          0.0, 0.0,   z, 0.0,
		          0.0, 0.0, 0.0, 1.0)
	}

	// FIXME: proportion between - and +
	pub fn depth(&self, Position { z, .. }: Position) -> Mat4<f32> {
		assert!(z >= -100.0 && z <= 100.0);

		if z == 0.0 {
			return na::new_identity(4);
		}

		let factor = if z > 0.0 {
			1.0 + z / 100.0
		}
		else {
			(100.0 - z.abs()) / 100.0
		};

		Mat4::new(factor,    0.0,    0.0, 0.0,
		             0.0, factor,    0.0, 0.0,
		             0.0,    0.0, factor, 0.0,
		             0.0,    0.0,    0.0, 1.0)
	}

	pub fn rotation(&self, roll: f32, pitch: f32, yaw: f32) -> Mat4<f32> {
		na::to_homogeneous(&Rot3::new_with_euler_angles(roll, pitch, yaw))
	}
}
