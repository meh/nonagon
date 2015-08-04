use std::collections::HashSet;

use glium::glutin::Event;
use glium::glutin::ElementState::{Pressed, Released};
use glium::glutin::VirtualKeyCode as Key;

use ffmpeg::Rational;

use util::Aspect;
use config;
use analyzer::Analyzer;
use game::{Update, Alive, Support, Position, Player, Ship, Projectile, Particle, Events};

#[derive(Debug)]
pub struct State {
	player:      Player,
	enemies:     Vec<Ship>,
	projectiles: Vec<Projectile>,
	particles:   Vec<Particle>,

	config: config::Game,
	aspect: Rational,
	keys:   HashSet<Key>,
	tick:   usize,
	events: Events,
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
			player:      player,
			enemies:     Vec::new(),
			projectiles: Vec::new(),
			particles:   Vec::new(),

			config: config.clone(),
			aspect: aspect.reduce(),
			keys:   HashSet::new(),
			tick:   0,
			events: Events::new(),
		}
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

	pub fn projectiles(&self) -> &[Projectile] {
		&self.projectiles
	}

	pub fn particles(&self) -> &[Particle] {
		&self.particles
	}

	pub fn tick(&mut self, time: f64, analyzer: &mut Analyzer) {
		// Fetch events from the analyzer.
		self.events.fetch(analyzer);

		// Deal with the player.
		{
			// Reset the player state.
			self.player.reset();

			// Update player positional velocity.
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

			// Update player rotational velocity.
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
		}

		// Update the state.
		{
			// Create the support.
			let support = Support::new(&self.config, self.aspect, self.tick, time, &self.events);

			// Update the player state.
			self.player.update(&support);

			// Update the enemies state.
			for enemy in &mut self.enemies {
				enemy.update(&support);
			}

			// Update the projectiles state.
			for projectile in &mut self.projectiles {
				projectile.update(&support);
			}

			// Update the particles state.
			for particle in &mut self.particles {
				particle.update(&support);
			}

			// Keep only alive enemies.
			self.enemies.retain(|e| e.alive(&support));

			// Keep only alive projectiles.
			self.projectiles.retain(|p| p.alive(&support));

			// Keep only alive particles.
			self.particles.retain(|p| p.alive(&support));
		}

		// Increase the current tick.
		self.tick += 1;
	}
}
