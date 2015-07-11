use glium::{Display, Surface};

use game;
use renderer::{Render, Support};

mod dot;
use self::dot::Dot;

pub struct Particle<'a> {
	display: &'a Display,

	dot: Dot<'a>,
}

impl<'a> Particle<'a>{
	pub fn new<'b>(display: &'b Display) -> Particle<'b> {
		Particle {
			display: display,

			dot: Dot::new(display),
		}
	}
}

impl<'a> Render<game::Particle> for Particle<'a> {
	fn render<S: Surface>(&mut self, target: &mut S, support: &Support, state: &game::Particle) {
		match state {
			&game::Particle::Dot { .. } =>
				self.dot.render(target, support, state),
		}
	}
}
