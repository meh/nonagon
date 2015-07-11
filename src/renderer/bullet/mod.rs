use glium::{Display, Surface};

use game;
use renderer::{Render, Support};

mod plasma;
use self::plasma::Plasma;

mod ray;
use self::ray::Ray;

pub struct Bullet<'a> {
	display: &'a Display,

	plasma: Plasma<'a>,
	ray:    Ray<'a>,
}

impl<'a> Bullet<'a>{
	pub fn new<'b>(display: &'b Display) -> Bullet<'b> {
		Bullet {
			display: display,

			plasma: Plasma::new(display),
			ray:    Ray::new(display),
		}
	}
}

impl<'a> Render<game::Bullet> for Bullet<'a> {
	fn render<S: Surface>(&mut self, target: &mut S, support: &Support, state: &game::Bullet) {
		match state {
			&game::Bullet::Plasma { .. } =>
				self.plasma.render(target, support, state),

			&game::Bullet::Ray { .. } =>
				self.ray.render(target, support, state),
		}
	}
}
