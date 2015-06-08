mod buffered;
pub use self::buffered::Buffered;

mod music;
pub use self::music::Music;

use openal::{Error, Device, Context};

use state::State;

pub struct Sound<'a> {
	device:  Device<'a>,
	context: Context<'a>,
}

impl<'a> Sound<'a> {
	pub fn new() -> Result<Self, Error> {
		let     device  = try!(Device::open(None));
		let mut context = try!(Context::new(&device));

		try!(context.make_current());

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
