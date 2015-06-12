use glium::{Program, Display, VertexBuffer, IndexBuffer, Surface};
use glium::index::PrimitiveType;

use ::game;

pub struct Cube<'a> {
	display: &'a Display,
	program: Program,
	indices: IndexBuffer<u16>,
}

impl<'a> Cube<'a> {
	pub fn new<'b>(display: &'b Display) -> Cube<'b> {
		let program = program!(display,
			110 => {
				vertex: "
					#version 110

					attribute vec3 position;

					void main() {
						gl_Position = vec4(position, 1.0);
					}
				",

				fragment: "
					#version 110

					void main() {
						gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
					}
				",
			}).unwrap();

		let indices = IndexBuffer::new(display, PrimitiveType::TriangleStrip, vec![1, 2, 0, 3]);

		Cube {
			display: display,
			program: program,
			indices: indices,
		}
	}

	pub fn render<T: Surface>(&self, target: &mut T, state: &game::Ship) {

	}
}

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
}

implement_vertex!(Vertex, position);
