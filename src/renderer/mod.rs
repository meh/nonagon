use ffmpeg::frame;
use glium::{Display, Surface};

mod video;
use self::video::Video;

mod ship;
use self::ship::Ship;

use na::{self, Persp3, Mat4, Vec3, Pnt3, Iso3, ToHomogeneous};

use ::game;

pub struct Renderer<'a> {
	display: &'a Display,
	view:    Mat4<f32>,

	video: Video<'a>,
	ship:  Ship<'a>,
}

impl<'a> Renderer<'a> {
	pub fn new<'b>(display: &'b Display) -> Renderer<'b> {
		Renderer {
			display: display,
			view:    na::new_identity(4),

			video: Video::new(display),
			ship:  Ship::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		let perspective = Persp3::new(width as f32 / height as f32, 45.0, 0.1, 20.0);

		let mut view = Iso3::new(na::zero(), na::zero());
		view.look_at_z(
			&Pnt3::new(0.0, 0.0,  0.0),
			&Pnt3::new(0.0, 0.0, -8.0),
			&Vec3::new(0.0, 1.0,  0.0));

		self.view = perspective.to_mat() * view.to_homogeneous();
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, state: &game::State, frame: Option<&frame::Video>) {
		if let Some(frame) = frame {
			self.video.render(target, frame);
		}
	}
}
