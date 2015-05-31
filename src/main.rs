#![feature(plugin)]
#![plugin(docopt_macros)]
#![allow(dead_code)]

extern crate ffmpeg;
use ffmpeg::time;

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};
use glium::glutin;

extern crate cpal;

extern crate image;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate docopt;
extern crate rustc_serialize;

use std::process::exit;

mod source;
use source::Source;

mod state;
use state::State;

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

	let mut source = Source::new(args.arg_source.clone()).unwrap_or_else(|err| {
		println!("error: ffmpeg: {}", err);
		exit(1);
	});

	if source.audio().is_none() {
		println!("error: the file has no audio");
		exit(2);
	}

	let display = glutin::WindowBuilder::new()
		.with_vsync()
		.with_title(String::from("nonagon"))
		.with_dimensions(640, 360)
		.build_glium()
		.unwrap();

	let mut renderer = Renderer::new(&display);
	let mut state    = State::new();

	let mut previous = time::relative() as f64 / 1_000_000.0;
	let mut lag      = 0.0;

	loop {
		let current = time::relative() as f64 / 1_000_000.0;
		let elapsed = current - previous;

		previous  = current;
		lag      += elapsed;

		for event in display.poll_events() {
			match event {
				glutin::Event::Closed => exit(0),
				_ => ()
			}
		}

		while lag >= GRANULARITY {
			source.sync();
			state.update();
			lag -= GRANULARITY;
		}

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);

		renderer.render(&mut target, &state, source.video().and_then(|v|
			if v.is_done() {
				None
			}
			else {
				Some(v.frame())
			}));

		target.finish();
	}
}
