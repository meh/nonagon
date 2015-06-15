pub mod cube;
pub use self::cube::Cube;

use glium::{Display, Surface};
use na::Mat4;

use ::game;

pub struct Ship<'a> {
	cube: Cube<'a>,
}

impl<'a> Ship<'a>{
	pub fn new<'b>(display: &'b Display) -> Ship<'b> {
		Ship {
			cube: Cube::new(display),
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, view: &Mat4<f32>, state: &game::Ship) {
		match state.shape {
			game::ship::Shape::Cube =>
				self.cube.render(target, view, state),
		}
	}
}
