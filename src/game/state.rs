use std::collections::{VecDeque, HashSet};

use glium::glutin::Event;
use glium::glutin::ElementState::{Pressed, Released};
use glium::glutin::VirtualKeyCode as Key;

use ffmpeg::{frame, Rational};

use util::Aspect;
use config;
use game::{Update, Alive, Support, Position, Player, Ship, Bullet, Particle};

#[derive(Debug)]
pub struct State {
	player:    Player,
	enemies:   Vec<Ship>,
	bullets:   Vec<Bullet>,
	particles: Vec<Particle>,

	timestamp: i64,
	frames:    VecDeque<frame::Audio>,

	config: config::Game,
	aspect: Rational,
	keys:   HashSet<Key>,
	tick:   usize,
}

impl State {
	pub fn new(config: &config::Game, aspect: Rational) -> Self {
		let mut player = Player::default();

		player.shape = config.ship().shape();

		player.position = Position {
			x: (aspect.width() as f32 / 2.0),
			y: (aspect.height() as f32 - 20.0),
			z: 0.0,
		};

		if let Some(face) = config.ship().face() {
			player.face = face;
		}

		if let Some(border) = config.ship().border() {
			player.border = border;
		}

		debug!("{:#?}", player);

		State {
			player:    player,
			enemies:   Vec::new(),
			bullets:   Vec::new(),
			particles: Vec::new(),

			timestamp: -1,
			frames:    VecDeque::new(),

			config: config.clone(),
			aspect: aspect.reduce(),
			keys:   HashSet::new(),
			tick:   0,
		}
	}

	pub fn feed(&mut self, frame: &frame::Audio) {
		if self.timestamp >= frame.timestamp().unwrap() {
			return;
		}

		// FIXME: uncomment this when ready
		//self.frames.push_back(frame.clone());
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

	pub fn player(&self) -> &Ship {
		&self.player
	}

	pub fn enemies(&self) -> &[Ship] {
		&self.enemies
	}

	pub fn bullets(&self) -> &[Bullet] {
		&self.bullets
	}

	pub fn particles(&self) -> &[Particle] {
		&self.particles
	}

	pub fn tick(&mut self, time: f64) {
		let support = Support::new(self.config.clone(), self.aspect, self.tick, time);

		self.update(&support);

		// --
		if support.tick() % 32 == 0 {
			self.bullets.push(Bullet::Plasma {
				position: Default::default(),
				velocity: ::game::Velocity { x: 0.56, y: 1.0, .. Default::default() },
				radius:   2.0,
			});
		}
		// --

		self.tick += 1;
	}
}

impl Update for State {
	fn update(&mut self, support: &Support) {
		self.player.reset();

		// position
		if self.keys.contains(&Key::Left) {
			if self.aspect.is_vertical() {
				self.player.velocity.x -= 1.0;
			}
			else {
				self.player.velocity.y += 1.0;
			}
		}

		if self.keys.contains(&Key::Up) {
			if self.aspect.is_vertical() {
				self.player.velocity.y -= 1.0;
			}
			else {
				self.player.velocity.x -= 1.0;
			}
		}

		if self.keys.contains(&Key::Right) {
			if self.aspect.is_vertical() {
				self.player.velocity.x += 1.0;
			}
			else {
				self.player.velocity.y -= 1.0;
			}
		}

		if self.keys.contains(&Key::Down) {
			if self.aspect.is_vertical() {
				self.player.velocity.y += 1.0;
			}
			else {
				self.player.velocity.x += 1.0;
			}
		}

		// rotation
		if self.keys.contains(&Key::A) {
			self.player.velocity.pitch -= 1.0;
		}

		if self.keys.contains(&Key::Q) {
			self.player.velocity.yaw -= 1.0;
		}

		if self.keys.contains(&Key::W) {
			self.player.velocity.roll -= 1.0;
		}

		if self.keys.contains(&Key::E) {
			self.player.velocity.yaw += 1.0;
		}

		if self.keys.contains(&Key::D) {
			self.player.velocity.pitch += 1.0;
		}

		if self.keys.contains(&Key::S) {
			self.player.velocity.roll += 1.0;
		}

		// state
		self.player.update(support);

		for enemy in &mut self.enemies {
			enemy.update(support);
		}

		for bullet in &mut self.bullets {
			bullet.update(support);
		}

		for particle in &mut self.particles {
			particle.update(support);
		}

		self.enemies.retain(|b| b.alive(support));
		self.bullets.retain(|b| b.alive(support));
		self.particles.retain(|p| p.alive(support));
	}
}
