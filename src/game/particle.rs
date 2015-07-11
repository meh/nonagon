use util::{Fill, Aspect};
use super::{Update, Position, Velocity};

#[derive(Debug)]
pub enum Particle {
	Dot {
		fill:  Fill,

		position: Position,
		velocity: Velocity,
		scale:    f32,
	}
}

impl Update for Particle {
	fn update(&mut self, tick: usize, aspect: &Aspect) {
		#[inline(always)]
		fn up(value: f32, velocity: f32, min: f32, max: f32, around: bool) -> f32 {
			let new = value + velocity;

			if new + velocity < min {
				if around {
					max
				}
				else {
					min
				}
			}
			else if new > max {
				if around {
					min
				}
				else {
					max
				}
			}
			else {
				new
			}
		}

		match self {
			&mut Particle::Dot { ref mut position, ref velocity, .. } => {
				position.x = up(position.x, velocity.x,    0.0, aspect.width() as f32,  false);
				position.y = up(position.y, velocity.y,    0.0, aspect.height() as f32, false);
				position.z = up(position.z, velocity.z, -100.0, 100.0,                  false);
			},
		}
	}
}
