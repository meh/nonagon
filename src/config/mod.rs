use std::path::Path;
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

pub mod analyzer;
pub use self::analyzer::Analyzer;

pub mod audio;
pub use self::audio::Audio;

pub mod video;
pub use self::video::Video;

#[derive(Clone, Default, Debug)]
pub struct Config {
	input: Option<String>,

	game:     Game,
	analyzer: Analyzer,
	audio:    Audio,
	video:    Video,
}

impl Config {
	pub fn load(args: &ArgvMap) -> Result<Config, ParserError> {
		let mut config = Config::default();
		let     files  = args.get_vec("--config");

		config.input = Some(String::from(args.get_str("<input>")));

		if !files.is_empty() {
			for file in &files {
				try!(config.merge(args, &file));
			}
		}

		Ok(config)
	}

	pub fn merge<P: AsRef<Path>>(&mut self, args: &ArgvMap, path: P) -> Result<(), ParserError> {
		if let Ok(mut file) = File::open(path.as_ref()) {
			let mut string = String::new();
			file.read_to_string(&mut string).unwrap();

			let mut parser = Parser::new(&string);

			if let Some(toml) = parser.parse() {
				try!(self.game.load(args, &Value::Table(toml.clone())));
				try!(self.analyzer.load(args, &Value::Table(toml.clone())));
				try!(self.audio.load(args, &Value::Table(toml.clone())));
				try!(self.video.load(args, &Value::Table(toml.clone())));
			}
			else {
				return Err(parser.errors.pop().unwrap());
			}
		}
		else {
			expect!("file not found");
		}

		Ok(())
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

	pub fn analyzer(&self) -> &Analyzer {
		&self.analyzer
	}
}
