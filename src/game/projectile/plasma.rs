use util::Aspect;
use game::{Update, Alive, Support, Position, Velocity};

#[derive(Debug)]
pub enum Plasma {
	Static {
		radius: f32,

		position: Position,
		velocity: Velocity,
	},

	Pulsating {
		min:  f32,
		max:  f32,
		step: f32,

		radius: f32,

		position: Position,
		velocity: Velocity,
	},
}

impl Update for Plasma {
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
			&mut Plasma::Static { ref mut position, ref velocity, .. } => {
				position.x = up(position.x, velocity.x,    0.0, support.aspect().width() as f32);
				position.y = up(position.y, velocity.y,    0.0, support.aspect().height() as f32);
				position.z = up(position.z, velocity.z, -100.0, 100.0);
			},

			&mut Plasma::Pulsating { ref mut position, ref velocity, min, max, ref mut step, ref mut radius, .. } => {
				position.x = up(position.x, velocity.x,    0.0, support.aspect().width() as f32);
				position.y = up(position.y, velocity.y,    0.0, support.aspect().height() as f32);
				position.z = up(position.z, velocity.z, -100.0, 100.0);

				if *radius == max || *radius == min {
					*step = -*step;
				}

				if *radius + *step > max {
					*radius = max;
				}
				else if *radius + *step < min {
					*radius = min;
				}
				else {
					*radius += *step;
				}
			},
		}
	}
}

impl Alive for Plasma {
	fn alive(&self, support: &Support) -> bool {
		match self {
			&Plasma::Static { position, velocity, .. } | &Plasma::Pulsating { position, velocity, .. } => {
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
