#![feature(plugin, core)]
#![plugin(docopt_macros)]
#![allow(dead_code)]

use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate ffmpeg;
use ffmpeg::time;

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};
use glium::glutin;

extern crate openal;

extern crate image;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate docopt;
extern crate rustc_serialize;

mod source;

mod state;
use state::State;

mod sound;
use sound::Sound;

mod renderer;
use renderer::Renderer;

docopt!(Args derive Debug, "
Usage: nonagon [options] <source>
       nonagon --help

Options:
  -h, --help    Show this message.
");

const GRANULARITY: f64 = 0.015;

fn main() {
	env_logger::init().unwrap();
	ffmpeg::init().unwrap();

	let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

	let (a, v) = source::spawn(args.arg_source.clone());

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
		let current = time::relative() as f64 / 1_000_000.0;
		let elapsed = current - previous;

		previous  = current;
		lag      += elapsed;

		for event in display.poll_events() {
			match event {
				glutin::Event::Closed => break 'game,

				_ => ()
			}
		}

		while lag >= GRANULARITY {
			if let Some(video) = video.as_mut() {
				video.sync();
			}

			state.lock().unwrap().update();

			lag -= GRANULARITY;
		}

		sound.render(&state.lock().unwrap());

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);

		renderer.render(&mut target, &state.lock().unwrap(), video.as_ref().and_then(|v|
			if v.is_done() {
				None
			}
			else {
				Some(v.frame())
			}));

		target.finish();
	}
}
