#![allow(dead_code, unused_variables)]

use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate ffmpeg;
use ffmpeg::time;

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};
use glium::glutin::{self, Event};
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::Escape;
use glium::glutin::get_primary_monitor;

extern crate openal;

extern crate nalgebra as na;

extern crate image;

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

mod util;

const GRANULARITY: f64 = 0.015;

static USAGE: &'static str = "
Usage: nonagon [options] <input>
       nonagon (-h | --help)
       nonagon (-v | --version)

Options:
	-h --help       Show this message.
	-v --version    Show version.

	--no-video      Do not show the video.
	--no-audio      Do not play the sound.
";

fn main() {
	env_logger::init().unwrap();
	ffmpeg::init().unwrap();

	let args = Docopt::new(USAGE).
		and_then(|d| d.parse()).
		unwrap_or_else(|e| e.exit());

	let no_audio = args.get_bool("--no-audio");
	let no_video = args.get_bool("--no-video");

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

	let (width, height) = {
		let (width, height) = get_primary_monitor().get_dimensions();

		match video.as_ref() {
			Some(video) if !no_video => {
				let w: u32 = width - 300;
				let h: u32 = w * video.height() / video.width();

				(w, h)
			},

			_ => {
				let h: u32 = height - 100;
				let w: u32 = h * 480 / 640;

				(w, h)
			}
		}
	};

	let display = glutin::WindowBuilder::new()
		.with_title(String::from("nonagon"))
		.with_dimensions(width, height)
		.with_depth_buffer(24)
		.with_vsync()
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

				if !no_audio {
					music.play(audio.frame());
				}

				state.lock().unwrap().feed(audio.frame());

				if next > 0.0 {
					time::sleep((next * 1_000_000.0) as u32).unwrap();
				}
			}
		});
	}

	let mut renderer = Renderer::new(&display);
	renderer.resize(width, height);

	let mut previous = time::relative() as f64 / 1_000_000.0;
	let mut lag      = 0.0;

	'game: loop {
		let mut current = time::relative() as f64 / 1_000_000.0;
		let mut elapsed = current - previous;

		// if the lag is smaller than granularity we wouldn't be doing anything, so
		// sleep for about the left-over time
		if lag + elapsed < GRANULARITY {
			time::sleep(((GRANULARITY - lag + elapsed) * 1_000_000.0) as u32 - 1_000).unwrap();

			current = time::relative() as f64 / 1_000_000.0;
			elapsed = current - previous;
		}

		previous  = current;
		lag      += elapsed;

		for event in display.poll_events() {
			match event {
				Event::Awakened => (),
				Event::Refresh  => (),

				Event::Closed | Event::KeyboardInput(Released, _, Some(Escape)) =>
					break 'game,

				Event::Resized(width, height) =>
					renderer.resize(width, height),

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

		sound.render(&state.lock().unwrap());

		let mut target = display.draw();
		target.clear_all((1.0, 1.0, 1.0, 1.0), 1.0, 0);

		renderer.render(&mut target, &state.lock().unwrap(), video.as_ref().and_then(|v|
			if no_video || v.is_done() {
				None
			}
			else {
				Some(v.frame())
			}));

		target.finish();
	}
}
