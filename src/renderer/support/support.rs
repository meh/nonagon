use std::rc::Rc;

use glium::Display;
use glium::texture::Texture2d;

use ffmpeg::Rational;

use settings;
use renderer::support::{Scene, Assets, Debug};

pub struct Support<'a> {
	display:  &'a Display,
	settings: settings::Video,

	background: Option<Rc<Texture2d>>,

	debug:  Debug,
	scene:  Scene,
	assets: Assets<'a>,
}

impl<'a> Support<'a> {
	pub fn new(display: &'a Display, settings: &settings::Video, aspect: Rational) -> Self {
		Support {
			display:  display,
			settings: settings.clone(),

			background: None,

			debug:  Debug::new(),
			scene:  Scene::new(aspect),
			assets: Assets::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.scene.resize(width, height);
	}

	pub fn update(&mut self, time: f64, background: Rc<Texture2d>) {
		self.debug.update(time);
		self.background = Some(background);
	}

	pub fn settings(&self) -> &settings::Video {
		&self.settings
	}

	pub fn debug(&self) -> &Debug {
		&self.debug
	}

	pub fn scene(&self) -> &Scene {
		&self.scene
	}

	pub fn assets(&self) -> &Assets {
		&self.assets
	}
}

impl<'a> AsRef<Texture2d> for Support<'a> {
	fn as_ref(&self) -> &Texture2d {
		&*self.background.as_ref().unwrap()
	}
}
