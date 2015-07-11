use ffmpeg::{frame, Rational};
use glium::{Display, Surface};

use game;
use config;
use renderer::{Render, Support, Video, Ship, Bullet, Particle};

pub struct Renderer<'a> {
	display: &'a Display,
	support: Support<'a>,

	video:    Video<'a>,
	ship:     Ship<'a>,
	bullet:   Bullet<'a>,
	particle: Particle<'a>,
}

impl<'a> Renderer<'a> {
	pub fn new<'b>(display: &'b Display, config: &config::Video, aspect: Rational) -> Renderer<'b> {
		Renderer {
			display: display,
			support: Support::new(display, config, aspect),

			video:    Video::new(display),
			ship:     Ship::new(display),
			bullet:   Bullet::new(display),
			particle: Particle::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.support.resize(width, height);
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, state: &game::State, frame: Option<&frame::Video>) {
		if let Some(frame) = frame {
			self.video.render(target, &self.support, frame);
		}

		self.ship.render(target, &self.support, state.player());

		for enemy in state.enemies() {
			self.ship.render(target, &self.support, enemy);
		}

		for bullet in state.bullets() {
			self.bullet.render(target, &self.support, bullet);
		}

		for particle in state.particles() {
			self.particle.render(target, &self.support, particle);
		}
	}
}
