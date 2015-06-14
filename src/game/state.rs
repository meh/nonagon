use glium::glutin::Event;

use ffmpeg::frame;

pub struct State {
	x: i32,
}

unsafe impl Sync for State { }

impl State {
	pub fn new() -> Self {
		State {
			x: 0,
		}
	}

	pub fn feed(&mut self, frame: &frame::Audio) {

	}

	pub fn handle(&mut self, event: &Event) {
		debug!("event:{:?}", event);
	}

	pub fn update(&mut self) {

	}
}
