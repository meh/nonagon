use ffmpeg::frame;

pub struct State {
	x: i32,
}

impl State {
	pub fn new() -> Self {
		State {
			x: 0,
		}
	}

	pub fn feed(&mut self, frame: &frame::Audio) {

	}

	pub fn update(&mut self) {

	}
}
