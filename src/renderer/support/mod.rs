mod scene;
pub use self::scene::Scene;

mod assets;
pub use self::assets::Assets;

use glium::Display;
use ffmpeg::Rational;

use config;

pub struct Support<'a> {
	config: config::Video,
	scene:  Scene,
	assets: Assets<'a>,
}

impl<'a> Support<'a> {
	pub fn new(display: &'a Display, config: &config::Video, aspect: Rational) -> Self {
		Support {
			config: config.clone(),
			scene:  Scene::new(aspect),
			assets: Assets::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.scene.resize(width, height);
	}

	pub fn config(&self) -> &config::Video {
		&self.config
	}

	pub fn scene(&self) -> &Scene {
		&self.scene
	}

	pub fn assets(&self) -> &Assets {
		&self.assets
	}
}
