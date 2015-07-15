use util::Aspect;
use game::{Update, Alive, Support, Position, Orientation, Velocity};

#[derive(Debug)]
pub enum Ray {
	Static {
		start:    f64,
		duration: f64,

		width: f32,

		position:    Position,
		orientation: Orientation,
		velocity:    Velocity,
	},

	Pulsating {
		start:    f64,
		duration: f64,

		min:  f32,
		max:  f32,
		step: f32,

		width: f32,

		position:    Position,
		orientation: Orientation,
		velocity:    Velocity,
	},
}

impl Update for Ray {
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
			&mut Ray::Static { ref mut position, ref mut orientation, ref velocity, .. } => {
				position.x = up(position.x, velocity.x,    0.0, support.aspect().width() as f32,  false);
				position.y = up(position.y, velocity.y,    0.0, support.aspect().height() as f32, false);
				position.z = up(position.z, velocity.z, -100.0, 100.0,                           false);

				orientation.roll  = up(orientation.roll,  velocity.roll,  0.0, 360.0, true);
				orientation.pitch = up(orientation.pitch, velocity.pitch, 0.0, 360.0, true);
				orientation.yaw   = up(orientation.yaw,   velocity.yaw,   0.0, 360.0, true);
			},

			&mut Ray::Pulsating { ref mut position, ref mut orientation, ref velocity, min, max, ref mut step, ref mut width, .. } => {
				position.x = up(position.x, velocity.x,    0.0, support.aspect().width() as f32,  false);
				position.y = up(position.y, velocity.y,    0.0, support.aspect().height() as f32, false);
				position.z = up(position.z, velocity.z, -100.0, 100.0,                           false);

				orientation.roll  = up(orientation.roll,  velocity.roll,  0.0, 360.0, true);
				orientation.pitch = up(orientation.pitch, velocity.pitch, 0.0, 360.0, true);
				orientation.yaw   = up(orientation.yaw,   velocity.yaw,   0.0, 360.0, true);

				if *width == max || *width == min {
					*step = -*step;
				}

				if *width + *step > max {
					*width = max;
				}
				else if *width + *step < min {
					*width = min;
				}
				else {
					*width += *step;
				}
			},
		}
	}
}

impl Alive for Ray {
	fn alive(&self, support: &Support) -> bool {
		match self {
			&Ray::Static { start, duration, .. } | &Ray::Pulsating { start, duration, .. } =>
				support.time() - start < duration,
		}
	}
}
