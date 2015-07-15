mod plasma;
pub use self::plasma::Plasma;

mod ray;
pub use self::ray::Ray;

use game::{Update, Alive, Support};

#[derive(Debug)]
pub enum Projectile {
	Plasma(Plasma),
	Ray(Ray),
}

impl Update for Projectile {
	fn update(&mut self, support: &Support) {
		match self {
			&mut Projectile::Plasma(ref mut p) =>
				p.update(support),

			&mut Projectile::Ray(ref mut r) =>
				r.update(support),
		}
	}
}

impl Alive for Projectile {
	fn alive(&self, support: &Support) -> bool {
		match self {
			&Projectile::Plasma(ref p) =>
				p.alive(support),

			&Projectile::Ray(ref r) =>
				r.alive(support),
		}
	}
}
