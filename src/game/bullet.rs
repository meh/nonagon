use util::Aspect;
use game::{Update, Support, Position, Orientation, Velocity};

#[derive(Debug)]
pub enum Bullet {
	Plasma {
		position: Position,
		velocity: Velocity,
		radius:   f32,
	},

	Ray {
		width:    f32,
		duration: f64,
		start:    f64,

		position:    Position,
		orientation: Orientation,
		velocity:    Velocity,
	}
}

impl Update for Bullet {
	fn update(&mut self, support: &Support) {
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
			&mut Bullet::Plasma { ref mut position, ref velocity, .. } => {
				position.x = up(position.x, velocity.x,    0.0, support.aspect().width() as f32,  false);
				position.y = up(position.y, velocity.y,    0.0, support.aspect().height() as f32, false);
				position.z = up(position.z, velocity.z, -100.0, 100.0,                  false);
			},

			&mut Bullet::Ray { ref mut position, ref mut orientation, ref velocity, .. } => {
				position.x = up(position.x, velocity.x,    0.0, support.aspect().width() as f32,  false);
				position.y = up(position.y, velocity.y,    0.0, support.aspect().height() as f32, false);
				position.z = up(position.z, velocity.z, -100.0, 100.0,                  false);

				orientation.roll  = up(orientation.roll,  velocity.roll,  0.0, 360.0, true);
				orientation.pitch = up(orientation.pitch, velocity.pitch, 0.0, 360.0, true);
				orientation.yaw   = up(orientation.yaw,   velocity.yaw,   0.0, 360.0, true);
			}
		}
	}
}
