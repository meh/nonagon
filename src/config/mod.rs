use std::fs::File;
use std::io::Read;

use docopt::ArgvMap;
use toml::{Parser, ParserError, Value};

macro_rules! expect {
	($msg:expr) => (
		return Err(::toml::ParserError {
			lo: 0,
			hi: 0,

			desc: String::from($msg),
		})
	);

	($pred:expr, $msg:expr) => ({
		if let Some(value) = $pred {
			value
		}
		else {
			return Err(::toml::ParserError {
				lo: 0,
				hi: 0,

				desc: String::from($msg),
			})
		}
	});
}

pub trait Load {
	fn load(&mut self, args: &ArgvMap, table: &Value) -> Result<(), ParserError> {
		Ok(())
	}
}

pub mod game;
pub use self::game::Game;

pub mod audio;
pub use self::audio::Audio;

#[macro_use]
pub mod video;
pub use self::video::Video;

#[derive(Clone, Default, Debug)]
pub struct Config {
	input: Option<String>,

	game:  Game,
	audio: Audio,
	video: Video,
}

impl Config {
	pub fn load(args: &ArgvMap) -> Result<Config, ParserError> {
		let mut config = Config::default();

		config.input = Some(String::from(args.get_str("<input>")));

		if !args.get_str("--config").is_empty() {
			if let Ok(mut file) = File::open(&args.get_str("--config")) {
				let mut string = String::new();
				file.read_to_string(&mut string).unwrap();

				let mut parser = Parser::new(&string);

				if let Some(toml) = parser.parse() {
					try!(config.game.load(args, &Value::Table(toml.clone())));
					try!(config.audio.load(args, &Value::Table(toml.clone())));
					try!(config.video.load(args, &Value::Table(toml.clone())));
				}
				else {
					return Err(parser.errors.pop().unwrap());
				}
			}
			else {
				expect!("file not found");
			}
		}

		Ok(config)
	}

	pub fn input(&self) -> &str {
		self.input.as_ref().unwrap()
	}

	pub fn game(&self) -> &Game {
		&self.game
	}

	pub fn audio(&self) -> &Audio {
		&self.audio
	}

	pub fn video(&self) -> &Video {
		&self.video
	}
}
