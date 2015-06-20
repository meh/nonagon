mod music;
pub use self::music::Music;

use openal::{Error, Device, Context, context};

use game::State;

pub struct Sound<'a> {
	device:  Device<'a>,
	context: context::Current<'a>,
}

impl<'a> Sound<'a> {
	pub fn new() -> Result<Self, Error> {
		let device  = try!(Device::default());
		let context = try!(try!(Context::new(&device)).into_current());

		Ok(Sound {
			device:  device,
			context: context,
		})
	}

	pub fn music(&self) -> Music {
		Music::new()
	}

	pub fn render(&mut self, state: &State) {

	}
}
