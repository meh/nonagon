#![allow(dead_code)]

use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate ffmpeg;
use ffmpeg::time;

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};
use glium::glutin::{self, Event};

extern crate openal;

extern crate image;

extern crate nalgebra as na;

extern crate docopt;
use docopt::Docopt;

#[macro_use]
extern crate log;
extern crate env_logger;

mod source;

mod game;
use game::State;

mod sound;
use sound::Sound;

mod renderer;
use renderer::Renderer;

const GRANULARITY: f64 = 0.015;

static USAGE: &'static str = "
Usage: nonagon [options] <input>
       nonagon (-h | --help)
       nonagon --version

Options:
	-h --help        Show this message.
	--version        Show version.
	-n --no-video    Ignore the video part.
";

fn main() {
	env_logger::init().unwrap();
	ffmpeg::init().unwrap();

	let args = Docopt::new(USAGE).
		and_then(|d| d.parse()).
		unwrap_or_else(|e| e.exit());

	let (a, v) = source::spawn(args.get_str("<input>"));

	let mut audio = match a {
		Err(error) => {
			println!("error: ffmpeg: {}", error);
			exit(1);
		},

		Ok(None) => {
			println!("error: the file has no audio");
			exit(2);
		},

		Ok(Some(a)) =>
			a
	};

	let mut video = match v {
		Err(error) => {
			println!("error: ffmpeg: {}", error);
			exit(3);
		},

		Ok(v) =>
			v
	};

	let display = glutin::WindowBuilder::new()
		.with_vsync()
		.with_title(String::from("nonagon"))
		.with_dimensions(640, 360)
		.build_glium()
		.unwrap();

	let mut sound = Sound::new().unwrap_or_else(|err| {
		println!("error: sound: {}", err);
		exit(3);
	});

	let state = Arc::new(Mutex::new(State::new()));

	{
		let     state = state.clone();
		let mut music = sound.music();

		thread::spawn(move || {
			loop {
				let next = audio.sync();

				music.play(audio.frame());
				state.lock().unwrap().feed(audio.frame());

				time::sleep((next * 1_000_000.0) as u32).unwrap();
			}
		});
	}

	let mut renderer = Renderer::new(&display);
	let mut previous = time::relative() as f64 / 1_000_000.0;
	let mut lag      = 0.0;

	'game: loop {
		let mut state   = state.lock().unwrap();
		let     current = time::relative() as f64 / 1_000_000.0;
		let     elapsed = current - previous;

		previous  = current;
		lag      += elapsed;

		for event in display.poll_events() {
			match event {
				Event::Awakened => (),
				Event::Refresh  => (),

				Event::Closed => {
					break 'game;
				},

				Event::Resized(width, height) => {
					debug!("resized: {}x{}", width, height);
				},

				Event::Moved(x, y) => {
					debug!("moved: {}:{}", x, y);
				},

				Event::Focused(true) => {
					debug!("focused");
				},

				Event::Focused(false) => {
					debug!("defocused");
				},

				event => state.handle(&event)
			}
		}

		while lag >= GRANULARITY {
			if let Some(video) = video.as_mut() {
				video.sync();
			}

			state.update();

			lag -= GRANULARITY;
		}

		sound.render(&state);

		let mut target = display.draw();
		target.clear_color(1.0, 1.0, 1.0, 1.0);

		renderer.render(&mut target, &state, video.as_ref().and_then(|v|
			if args.get_bool("--no-video") || v.is_done() {
				None
			}
			else {
				Some(v.frame())
			}));

		target.finish();
	}
}
