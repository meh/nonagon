use util::Aspect;
use game::{Update, Alive, Support, Position, Velocity};

#[derive(Debug)]
pub enum Particle {
	Dot {
		position: Position,
		velocity: Velocity,
		scale:    f32,
	}
}

impl Update for Particle {
	fn update(&mut self, support: &Support) {
		#[inline(always)]
		fn up(value: f32, velocity: f32, min: f32, max: f32) -> f32 {
			let new = value + velocity;

			if new + velocity < min {
				min
			}
			else if new > max {
				max
			}
			else {
				new
			}
		}

		match self {
			&mut Particle::Dot { ref mut position, ref velocity, .. } => {
				position.x = up(position.x, velocity.x,    0.0, support.aspect().width() as f32);
				position.y = up(position.y, velocity.y,    0.0, support.aspect().height() as f32);
				position.z = up(position.z, velocity.z, -100.0, 100.0);
			},
		}
	}
}

impl Alive for Particle {
	fn alive(&self, support: &Support) -> bool {
		match self {
			&Particle::Dot { position, velocity, .. } => {
				// if going against the right wall
				if position.x == support.aspect().width() as f32 && velocity.x > 0.0 {
					return false;
				}

				// if going against the left wall
				if position.x == 0.0 && velocity.x < 0.0 {
					return false;
				}

				// if going against the bottom wall
				if position.y == support.aspect().height() as f32 && velocity.y > 0.0 {
					return false;
				}

				// if going against the top wall
				if position.y == 0.0 && velocity.y < 0.0 {
					return false;
				}

				true
			}
		}
	}
}
