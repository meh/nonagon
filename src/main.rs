#![feature(plugin)]
#![plugin(docopt_macros)]
#![allow(dead_code)]

extern crate ffmpeg;

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

mod video;
use video::Video;

mod audio;
use audio::Audio;

docopt!(Args derive Debug, "
Usage: nonagon [options] <source>
       nonagon --help

Options:
  -h, --help    Show this message.
");

fn main() {
	env_logger::init().unwrap();
	ffmpeg::init().unwrap();

	let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

	let source = Source::new(args.arg_source.clone()).unwrap_or_else(|err| {
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

	let mut video = source.video().map(|v|
		Video::new(&display, v).unwrap_or_else(|err| {
			println!("error: ffmpeg: {}", err);
			exit(3);
		}));

	let mut audio = source.audio().map(|v|
		Audio::new(v).unwrap_or_else(|err| {
			println!("error: cpal: {}", err);
			exit(4);
		}));

	loop {
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);

		if let Some(video) = video.as_mut() {
			if !video.is_done() {
				video.sync();
				video.draw(&mut target);
			}
		}

		if let Some(audio) = audio.as_mut() {
			if !audio.is_done() {
				audio.sync();
				audio.play();
			}
		}

		target.finish();

		for event in display.poll_events() {
			match event {
				glutin::Event::Closed => exit(0),
				_ => ()
			}
		}
	}
}
