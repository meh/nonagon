mod cube;
pub use self::cube::Cube;

mod tetrahedron;
pub use self::tetrahedron::Tetrahedron;

use glium::{Display, Surface};

use game;
use renderer::Support;

pub struct Ship<'a> {
	cube:        Cube<'a>,
	tetrahedron: Tetrahedron<'a>,
}

impl<'a> Ship<'a>{
	pub fn new<'b>(display: &'b Display) -> Ship<'b> {
		Ship {
			cube:        Cube::new(display),
			tetrahedron: Tetrahedron::new(display),
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, support: &Support, state: &game::Ship) {
		match state.shape {
			game::ship::Shape::Cube =>
				self.cube.render(target, support, state),

			game::ship::Shape::Tetrahedron =>
				self.tetrahedron.render(target, support, state),
		}
	}
}
