use glium::glutin::Event;
use glium::glutin::ElementState::Pressed;
use glium::glutin::VirtualKeyCode::{Left, Up, Right, Down};

use ffmpeg::{frame, Rational};

use util::rgba;
use super::{Position, Direction, Velocity};
use super::ship::{self, Ship};
use super::bullet::{self, Bullet};

pub struct State {
	pub player:  Ship,
	pub enemies: Vec<Ship>,
	pub bullets: Vec<Bullet>,

	aspect: Rational,
}

unsafe impl Sync for State { }

impl State {
	pub fn new(aspect: Rational) -> Self {
		State {
			player: Ship {
				shape:     ship::Shape::Cube,
				position:  Position::new(0, 0),
				direction: Direction::new(0, 0),
				color:     rgba(255, 0, 0, 220),
			},

			enemies: Vec::new(),
			bullets: Vec::new(),

			aspect: aspect.reduce(),
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

			&Event::KeyboardInput(Pressed, _, Some(Left)) => {
				match self.player.position.x {
					0 => (),
					v => self.player.position.x -= 1,
				}
			},

			&Event::KeyboardInput(Pressed, _, Some(Up)) => {
				match self.player.position.y {
					0 => (),
					v => self.player.position.y -= 1,
				}
			}

			&Event::KeyboardInput(Pressed, _, Some(Right)) => {
				let max = if self.aspect == Rational::new(3, 4) {
					480
				}
				else {
					unimplemented!();
				};

				match self.player.position.x {
					v if v == max => (),
					v             => self.player.position.x += 1,
				}
			},

			&Event::KeyboardInput(Pressed, _, Some(Down)) => {
				let max = if self.aspect == Rational::new(3, 4) {
					640
				}
				else {
					unimplemented!();
				};

				match self.player.position.y {
					v if v == max => (),
					v             => self.player.position.y += 1,
				}
			},

			&Event::KeyboardInput(..) => (),

			_ => unreachable!()
		}
	}

	pub fn update(&mut self) {

	}
}
