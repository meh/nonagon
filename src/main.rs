#![allow(dead_code, unused_variables)]
#![feature(associated_type_defaults)]

use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

extern crate ffmpeg;
use ffmpeg::{time, Rational};

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};
use glium::SwapBuffersError::{ContextLost, AlreadySwapped};
use glium::glutin::{self, Event};
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::Escape;
use glium::glutin::get_primary_monitor;

extern crate openal;

extern crate toml;

extern crate bdf;

extern crate lzma;

extern crate image;

extern crate regex;

extern crate docopt;
use docopt::Docopt;

extern crate nalgebra as na;
extern crate ncollide as nc;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rft;

extern crate num;

extern crate strided;

#[macro_use]
mod util;

#[macro_use]
mod config;
use config::Config;

mod source;

mod game;
use game::State;

mod sound;
use sound::Sound;

mod renderer;
use renderer::Renderer;

mod analyzer;
use analyzer::Analyzer;

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
	// Initialize libraries.
	env_logger::init().unwrap();
	ffmpeg::init().unwrap();

	// Load the config from the given args.
	let config = Config::load(&Docopt::new(USAGE).
		and_then(|d| d.parse()).
		unwrap_or_else(|e| e.exit())).unwrap();

	debug!("{:#?}", config);

	// Spawn the source decoder.
	let (a, v) = source::spawn(config.input(), config.audio().only());

	// Check for errors for the audio decoder.
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

	// Check for errors for the video decoder.
	let mut video = match v {
		Err(error) => {
			println!("error: ffmpeg: {}", error);
			exit(3);
		},

		Ok(v) =>
			v
	};

	// Calculate the window size based on the monitor dimension.
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

	// Open the display with mandatory options.
	let mut display = glutin::WindowBuilder::new()
		.with_title(String::from("nonagon"))
		.with_dimensions(width, height)
		.with_depth_buffer(24);

	// Enable vsync if the config says so.
	if config.video().vsync() {
		display = display.with_vsync();
	}

	// Enable multisampling if the config says so.
	if let Some(value) = config.video().multisampling() {
		display = display.with_multisampling(value);
	}

	// Build the display.
	let display = display.build_glium().unwrap_or_else(|err| {
		println!("error: opengl: configuration not supported: {}", err);
		exit(4);
	});

	// Create the sound device.
	//
	// It's in an Arc<Mutex<_>> because it's accessed both from the main thread
	// and the music thread.
	let sound = Arc::new(Mutex::new(Sound::new(config.audio()).unwrap_or_else(|err| {
		println!("error: sound: {}", err);
		exit(5);
	})));

	// Create the analyzer.
	//
	// It's in an Arc<Mutex<_>> because it's accessed both from the main thread
	// and the music thread.
	let analyzer = Arc::new(Mutex::new(Analyzer::spawn(config.analyzer())));

	// Create the state to keep track of the game.
	let mut state = State::new(config.game(), aspect);

	// Music thread, the result is stored so it can be killed from the main later
	// on.
	let music = {
		let analyzer = analyzer.clone();
		let sound    = sound.clone();
		let mute     = config.audio().mute();

		// Channel for killing.
		let (sender, receiver) = channel();

		(sender, thread::spawn(move || {
			// Set to 0 because we set it as soon as we get the first frame so it
			// stays in sync.
			let mut start = 0.0;

			// Keeps track of how far in stream we got.
			let mut offset = 0.0;

			// How many seconds of samples we have.
			let mut duration = 0.0;

			loop {
				// Return if the main has exited or the stream is done.
				if receiver.try_recv().is_ok() {
					return;
				}

				// Get the next frame, if any.
				let frame = audio.next();

				// If there's no frame the stream is done.
				if frame.is_none() {
					return;
				}

				let frame = frame.unwrap();

				// Only play the sound if it's not muted.
				if !mute {
					sound.lock().unwrap().play(&frame);
				}

				// Initialize the start as soon as the first frame is played.
				if start == 0.0 {
					start = time::relative() as f64 / 1_000_000.0;
					analyzer.lock().unwrap().start(start);
				}

				// Increment by seconds of sample data we have.
				duration += (1.0 / 44100.0) * frame.samples() as f64;

				// Feed the frame to the analyzer.
				analyzer.lock().unwrap().feed(frame);

				// Return if the stream is over or main has exited.
				if receiver.try_recv().is_ok() {
					return;
				}

				// If we have 1 second and half worth of samples sleep for the
				// remaining duration.
				if duration >= 1.5 {
					// Add the current duration to the offset so we have a baseline to
					// correct.
					offset += duration;

					// We need the current time so we don't oversleep.
					let current = time::relative() as f64 / 1_000_000.0;

					// Correct the duration considering time that has passed since we
					// fetched the samples.
					let mut corrected = offset - (current - start);

					// Sleep in small portions so we can check for liveness.
					while corrected >= 0.1 {
						corrected -= 0.1;
						time::sleep((0.1 * 1_000_000.0) as u32).unwrap();

						// Return if the main has exited.
						if receiver.try_recv().is_ok() {
							return;
						}
					}

					// Do a final sleep in case the loop is done with a leftover.
					if corrected > 0.0 {
						time::sleep((corrected * 1_000_000.0) as u32).unwrap();
					}

					// Reset the duration for the next cycle.
					duration = 0.0;
				}
			}
		}))
	};

	// Create the renderer.
	let mut renderer = Renderer::new(&display, config.video(), aspect);

	// Give it the initial size.
	renderer.resize(width, height);

	// The static step between each state update.
	const GRANULARITY: f64 = 0.015;

	// The previous time.
	let mut previous = time::relative() as f64 / 1_000_000.0;

	// The accumulated lag.
	let mut lag = 0.0;

	'game: loop {
		let current = time::relative() as f64 / 1_000_000.0;
		let elapsed = current - previous;

		previous  = current;
		lag      += elapsed;

		// Fetch the events and handle them.
		for event in display.poll_events() {
			match event {
				// These do nothing, but we match on them so the state handle can be
				// partial.
				Event::Awakened => (),
				Event::Refresh  => (),
				Event::Moved(x, y)    => (),
				Event::Focused(true)  => (),
				Event::Focused(false) => (),

				// When the window is closed or ESC is pressed, quit the game.
				Event::Closed | Event::KeyboardInput(Released, _, Some(Escape)) =>
					break 'game,

				// The window has been resized.
				Event::Resized(w, h) => {
					// Cache the new dimension.
					width  = w;
					height = h;

					// Tell the renderer the new size.
					renderer.resize(width, height);
				},

				// Handle the supported events.
				event =>
					state.handle(&event)
			}
		}

		// Make sure the state gets updated in splits of `GRANULARITY` seconds.
		while lag >= GRANULARITY {
			// Run an update tick.
			state.tick(current - lag, &mut analyzer.lock().unwrap());

			lag -= GRANULARITY;
		}

		// Render the sounds effects.
		sound.lock().unwrap().render(&state);

		// If we have a video, sync it.
		if let Some(video) = video.as_mut() {
			video.sync();
		}

		// Prepare to render a frame.
		let mut target = display.draw();
		target.clear_all((1.0, 1.0, 1.0, 1.0), 1.0, 0);

		// Run the renderer.
		renderer.render(&mut target, current, &state, video.as_ref().and_then(|v|
			if v.is_done() {
				None
			}
			else {
				Some(v.frame())
			}));

		// Draw the frame.
		match target.finish() {
			// If we lost the context (it can happen) recreate the renderer.
			Err(ContextLost) => {
				renderer = Renderer::new(&display, config.video(), aspect);
				renderer.resize(width, height);
			},

			Err(AlreadySwapped) =>
				(),

			Ok(..) =>
				()
		}
	}

	// Ensure the music thread is closed.
	let _ = music.0.send(());
	music.1.join().unwrap();
}
