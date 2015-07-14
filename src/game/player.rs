use std::ops::{Deref, DerefMut};

use game::{Update, Support, Ship};

#[derive(Debug)]
pub struct Player {
	ship:  Ship,
	name:  Option<String>,
	score: u64,

	lives: u8,
}

impl Player {
	pub fn reset(&mut self) {
		self.velocity.x = 0.0;
		self.velocity.y = 0.0;
		self.velocity.z = 0.0;

		self.velocity.roll  = 0.0;
		self.velocity.pitch = 0.0;
		self.velocity.yaw   = 0.0;
	}
}

impl Default for Player {
	fn default() -> Player {
		Player {
			ship:  Default::default(),
			name:  None,
			score: 0,

			lives: 3,
		}
	}
}

impl Deref for Player {
	type Target = Ship;

	fn deref(&self) -> &Ship {
		&self.ship
	}
}

impl DerefMut for Player {
	fn deref_mut(&mut self) -> &mut Ship {
		&mut self.ship
	}
}

impl Update for Player {
	fn update(&mut self, support: &Support) {
		self.ship.update(support);
	}
}
