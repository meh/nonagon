use std::collections::HashSet;

use glium::glutin::Event;
use glium::glutin::ElementState::{Pressed, Released};
use glium::glutin::VirtualKeyCode as Key;

use ffmpeg::{frame, Rational};

use util::rgba;
use super::{Position, Direction, Orientation};
use super::ship::{self, Ship};
use super::bullet::{self, Bullet};

pub struct State {
	pub player:  Ship,
	pub enemies: Vec<Ship>,
	pub bullets: Vec<Bullet>,

	aspect: Rational,
	keys:   HashSet<Key>,
}

unsafe impl Sync for State { }

impl State {
	pub fn new(aspect: Rational) -> Self {
		State {
			player: Ship {
				shape:       ship::Shape::Cube,
				position:    Position(0, 0),
				direction:   Direction(0, 0),
				orientation: Orientation{ roll: 50.0, pitch: 110.0, yaw: 120.0 },
				color:       rgba(255, 0, 0, 220),
			},

			enemies: Vec::new(),
			bullets: Vec::new(),

			aspect: aspect.reduce(),
			keys:   HashSet::new(),
		}
	}

	pub fn feed(&mut self, frame: &frame::Audio) {

	}

	pub fn handle(&mut self, event: &Event) {
		match event {
			&Event::ReceivedCharacter(..) |
			&Event::MouseMoved(..) |
			&Event::MouseWheel(..) |
			&Event::MouseInput(..) => (),

			&Event::KeyboardInput(Pressed, _, Some(key)) => {
				self.keys.insert(key);
			},

			&Event::KeyboardInput(Released, _, Some(key)) => {
				self.keys.remove(&key);
			},

			&Event::KeyboardInput(..) => (),

			_ => unreachable!()
		}
	}

	pub fn update(&mut self) {
		if self.keys.contains(&Key::Left) {
			match self.player.position {
				Position(        0, _) => (),
				Position(ref mut x, _) => *x -= 1,
			}
		}

		if self.keys.contains(&Key::Up) {
			match self.player.position {
				Position(_,         0) => (),
				Position(_, ref mut y) => *y -= 1,
			}
		}

		if self.keys.contains(&Key::Right) {
			let max = if self.aspect == Rational(3, 4) {
				480
			}
			else {
				unimplemented!();
			};

			match self.player.position {
				Position(        x, _) if x == max => (),
				Position(ref mut x, _)             => *x += 1,
			}
		}

		if self.keys.contains(&Key::Down) {
			let max = if self.aspect == Rational(3, 4) {
				640
			}
			else {
				unimplemented!();
			};

			match self.player.position {
				Position(_,         y) if y == max => (),
				Position(_, ref mut y)             => *y += 1,
			}
		}

		if self.keys.contains(&Key::A) {
			match self.player.orientation {
				Orientation { ref mut pitch, .. } if *pitch == 0.0 =>
					*pitch = 360.0,

				Orientation { ref mut pitch, .. } =>
					*pitch -= 1.0,
			}
		}

		if self.keys.contains(&Key::Q) {
			match self.player.orientation {
				Orientation { ref mut yaw, .. } if *yaw == 0.0 =>
					*yaw = 360.0,

				Orientation { ref mut yaw, .. } =>
					*yaw -= 1.0,
			}
		}

		if self.keys.contains(&Key::W) {
			match self.player.orientation {
				Orientation { ref mut roll, .. } if *roll == 0.0 =>
					*roll = 360.0,

				Orientation { ref mut roll, .. } =>
					*roll -= 1.0,
			}
		}

		if self.keys.contains(&Key::E) {
			match self.player.orientation {
				Orientation { ref mut yaw, .. } if *yaw == 360.0 =>
					*yaw = 0.0,

				Orientation { ref mut yaw, .. } =>
					*yaw += 1.0,
			}
		}

		if self.keys.contains(&Key::D) {
			match self.player.orientation {
				Orientation { ref mut pitch, .. } if *pitch == 360.0 =>
					*pitch = 0.0,

				Orientation { ref mut pitch, .. } =>
					*pitch += 1.0,
			}
		}

		if self.keys.contains(&Key::S) {
			match self.player.orientation {
				Orientation { ref mut roll, .. } if *roll == 360.0 =>
					*roll = 0.0,

				Orientation { ref mut roll, .. } =>
					*roll += 1.0,
			}
		}
	}
}
