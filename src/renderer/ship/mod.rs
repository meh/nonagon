pub mod cube;
pub use self::cube::Cube;

use glium::{Display, Surface};

use game;
use renderer::Support;

pub struct Ship<'a> {
	cube: Cube<'a>,
}

impl<'a> Ship<'a>{
	pub fn new<'b>(display: &'b Display) -> Ship<'b> {
		Ship {
			cube: Cube::new(display),
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, support: &Support, state: &game::Ship) {
		match state.shape {
			game::ship::Shape::Cube =>
				self.cube.render(target, support, state),
		}
	}
}
