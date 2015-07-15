use glium::{Display, Surface};

use game;
use renderer::{Render, Support};

mod plasma;
use self::plasma::Plasma;

mod ray;
use self::ray::Ray;

pub struct Projectile<'a> {
	display: &'a Display,

	plasma: Plasma<'a>,
	ray:    Ray<'a>,
}

impl<'a> Projectile<'a>{
	pub fn new<'b>(display: &'b Display) -> Projectile<'b> {
		Projectile {
			display: display,

			plasma: Plasma::new(display),
			ray:    Ray::new(display),
		}
	}
}

impl<'a> Render<game::Projectile> for Projectile<'a> {
	fn render<S: Surface>(&mut self, target: &mut S, support: &Support, state: &game::Projectile) {
		match state {
			&game::Projectile::Plasma(ref p) =>
				self.plasma.render(target, support, p),

			&game::Projectile::Ray(ref r) =>
				self.ray.render(target, support, r),
		}
	}
}
