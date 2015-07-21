mod scene;
pub use self::scene::Scene;

mod assets;
pub use self::assets::Assets;

mod video;
pub use self::video::Video;

mod visualizer;
pub use self::visualizer::Visualizer;

use glium::Display;
use glium::texture::Texture2d;
use glium::framebuffer::SimpleFrameBuffer;

use ffmpeg::{frame, Rational};

use config;
use game;
use renderer::Render;

pub struct Support<'a> {
	display: &'a Display,
	config:  config::Video,
	time:    f64,

	background: Option<Texture2d>,
	video:      Video<'a>,
	visualizer: Visualizer<'a>,

	scene:      Scene,
	assets:     Assets<'a>,
}

impl<'a> Support<'a> {
	pub fn new(display: &'a Display, config: &config::Video, aspect: Rational) -> Self {
		Support {
			display: display,
			config:  config.clone(),
			time:    0.0,

			background: None,
			video:      Video::new(display),
			visualizer: Visualizer::new(display),

			scene:  Scene::new(aspect),
			assets: Assets::new(display),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.scene.resize(width, height);
	}

	pub fn update(&mut self, time: f64) {
		self.time = time;
	}

	pub fn background(&mut self, state: &game::State, frame: Option<&frame::Video>) {
		let texture = Texture2d::empty(self.display, self.scene().width(), self.scene().height()).unwrap();

		{
			let mut surface = SimpleFrameBuffer::new(self.display, &texture);

			if let Some(frame) = frame {
				self.video.render(&mut surface, self, frame);
			}
			else {
				self.visualizer.render(&mut surface, self, state);
			}
		}

		self.background = Some(texture);
	}

	pub fn config(&self) -> &config::Video {
		&self.config
	}

	pub fn time(&self) -> f64 {
		self.time
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
		self.background.as_ref().unwrap()
	}
}
