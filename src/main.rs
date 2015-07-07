#![allow(dead_code, unused_variables)]

use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

extern crate ffmpeg;
use ffmpeg::{time, Rational};

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};
use glium::SwapBuffersError::ContextLost;
use glium::glutin::{self, Event};
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::Escape;
use glium::glutin::get_primary_monitor;

extern crate openal;

extern crate toml;

extern crate image;

extern crate regex;

extern crate docopt;
use docopt::Docopt;

extern crate nalgebra as na;
extern crate ncollide as nc;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
mod util;

mod config;
use config::Config;

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
       nonagon (-v | --version)

Options:
	-h --help       Show this message.
	-v --version    Show version.

	-c --config PATH    The TOML configuration file.
	-a --audio-only     Do not show the video.
	-m --mute           Do not play the sound.
";

fn main() {
	env_logger::init().unwrap();
	ffmpeg::init().unwrap();

	let config = Config::load(&Docopt::new(USAGE).
		and_then(|d| d.parse()).
		unwrap_or_else(|e| e.exit())).unwrap();

	debug!("{:#?}", config);

	let (a, v) = source::spawn(config.input(), config.audio().only());

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

	let (mut width, mut height, aspect) = {
		let (width, height) = get_primary_monitor().get_dimensions();

		if let Some(video) = video.as_ref() {
			let w: u32 = width - 300;
			let h: u32 = w * video.height() / video.width();

			(w, h, Rational::new(video.width() as i32, video.height() as i32).reduce())
		}
		else {
			let h: u32 = height - 100;
			let w: u32 = h * 480 / 640;

			(w, h, Rational::new(480, 640).reduce())
		}
	};

	let mut display = glutin::WindowBuilder::new()
		.with_title(String::from("nonagon"))
		.with_dimensions(width, height)
		.with_srgb(Some(true))
		.with_depth_buffer(24);

	if config.video().vsync() {
		display = display.with_vsync();
	}

	if let Some(value) = config.video().multisampling() {
		display = display.with_multisampling(value);
	}

	let display = display.build_glium().unwrap_or_else(|err| {
		println!("error: opengl: configuration not supported");
		println!("{}", err);
		exit(4);
	});

	let sound = Arc::new(Mutex::new(Sound::new(config.audio()).unwrap_or_else(|err| {
		println!("error: sound: {}", err);
		exit(5);
	})));

	let state = Arc::new(Mutex::new(State::new(&config, aspect)));

	let music = {
		let state = state.clone();
		let sound = sound.clone();
		let play  = !config.audio().mute();

		let (sender, receiver) = channel();

		(sender, thread::spawn(move || {
			loop {
				let next = audio.sync();

				if play {
					sound.lock().unwrap().play(audio.frame());
				}

				state.lock().unwrap().feed(audio.frame());

				if next > 0.0 {
					time::sleep((next * 1_000_000.0) as u32).unwrap();
				}

				if audio.is_done() || receiver.try_recv().is_ok() {
					break;
				}
			}
		}))
	};

	let mut renderer = Renderer::new(&display, config.video(), aspect);
	renderer.resize(width, height);

	let mut previous = time::relative() as f64 / 1_000_000.0;
	let mut lag      = 0.0;

	'game: loop {
		let mut current = time::relative() as f64 / 1_000_000.0;
		let mut elapsed = current - previous;

		// if the lag is smaller than granularity we wouldn't be doing anything, so
		// sleep for about the left-over time
		if lag + elapsed < GRANULARITY {
			let time = ((GRANULARITY - lag + elapsed) * 1_000_000.0) as u32;

			if time > 5_000 {
				time::sleep(time - 5_000).unwrap();

				current = time::relative() as f64 / 1_000_000.0;
				elapsed = current - previous;
			}
		}

		previous  = current;
		lag      += elapsed;

		for event in display.poll_events() {
			match event {
				Event::Awakened => (),
				Event::Refresh  => (),

				Event::Closed | Event::KeyboardInput(Released, _, Some(Escape)) =>
					break 'game,

				Event::Resized(w, h) => {
					width  = w;
					height = h;

					renderer.resize(width, height);
				},

				Event::Moved(x, y) =>
					debug!("moved: {}:{}", x, y),

				Event::Focused(true) =>
					debug!("focused"),

				Event::Focused(false) =>
					debug!("defocused"),

				event =>
					state.lock().unwrap().handle(&event)
			}
		}

		while lag >= GRANULARITY {
			if let Some(video) = video.as_mut() {
				video.sync();
			}

			state.lock().unwrap().update();

			lag -= GRANULARITY;
		}

		sound.lock().unwrap().render(&state.lock().unwrap());

		let mut target = display.draw();
		target.clear_all((1.0, 1.0, 1.0, 1.0), 1.0, 0);

		renderer.render(&mut target, &state.lock().unwrap(), video.as_ref().and_then(|v|
			if v.is_done() {
				None
			}
			else {
				Some(v.frame())
			}));

		match target.finish() {
			Err(ContextLost) => {
				renderer = Renderer::new(&display, config.video(), aspect);
				renderer.resize(width, height);
			},

			Ok(..) => ()
		}
	}

	// ensure the music thread is closed
	let _ = music.0.send(());
	music.1.join().unwrap();
}
