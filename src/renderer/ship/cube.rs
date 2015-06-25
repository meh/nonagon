use glium::{Program, Display, VertexBuffer, IndexBuffer, Surface, DrawParameters};
use glium::DepthTest::{IfLess, IfLessOrEqual};
use glium::BlendingFunction::Addition;
use glium::LinearBlendingFactor::{SourceAlpha, OneMinusSourceAlpha};
use glium::BackfaceCullingMode::CullClockWise;
use glium::index::PrimitiveType::{TrianglesList, LinesList};

use util::rgb;
use game;
use renderer::Scene;

pub struct Cube<'a> {
	display: &'a Display,

	program:  Program,

	vertices: VertexBuffer<Vertex>,
	faces:    IndexBuffer<u16>,
	borders:  IndexBuffer<u16>,
}

impl<'a> Cube<'a> {
	pub fn new<'b>(display: &'b Display) -> Cube<'b> {
		Cube { display: display,
			program: program!(display,
				110 => {
					vertex: "
						#version 110

						attribute vec3 position;

						uniform mat4 mvp;

						void main() {
							gl_Position = mvp * vec4(position, 1.0);
						}
					",

					fragment: "
						#version 110

						uniform vec4 color;

						void main() {
							gl_FragColor = color;
						}
					",
				}).unwrap(),

			vertices: VertexBuffer::new(display, vec![
				// front
				Vertex { position: [-1.0, -1.0,  1.0] },
				Vertex { position: [ 1.0, -1.0,  1.0] },
				Vertex { position: [ 1.0,  1.0,  1.0] },
				Vertex { position: [-1.0,  1.0,  1.0] },

				// back
				Vertex { position: [-1.0, -1.0, -1.0] },
				Vertex { position: [ 1.0, -1.0, -1.0] },
				Vertex { position: [ 1.0,  1.0, -1.0] },
				Vertex { position: [-1.0,  1.0, -1.0] },
			]),

			faces: IndexBuffer::new(display, TrianglesList, vec![
				// front
				0, 1, 2,
				2, 3, 0,

				// back
				6, 5, 4,
				4, 7, 6,

				// bottom
				0, 4, 5,
				5, 1, 0,

				// top
				3, 2, 7,
				7, 2, 6,

				// right
				5, 2, 1,
				6, 2, 5,

				// left
				0, 3, 4,
				4, 3, 7,
			]),

			borders: IndexBuffer::new(display, LinesList, vec![
				// front
				0, 1,
				1, 2,
				2, 3,
				3, 0,

				// back
				4, 5,
				5, 6,
				6, 7,
				7, 4,

				// left
				0, 4,
				3, 7,

				// right
				1, 5,
				2, 6,
			]),
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, scene: &Scene, state: &game::Ship) {
		let mvp = scene.to_mat() *
			scene.position(state.position) *
			scene.orientation(state.orientation) *
			scene.scale(12.5);

		// draw the faces
		{
			let uniforms = uniform! {
				mvp:   mvp,
				color: state.color,
			};

			target.draw(&self.vertices, &self.faces, &self.program, &uniforms, &DrawParameters {
				backface_culling: CullClockWise,

				blending_function: Some(Addition {
					source:      SourceAlpha,
					destination: OneMinusSourceAlpha
				}),

				depth_test:  IfLess,
				depth_write: true,

				.. Default::default() }).unwrap();
		}

		// draw the borders
		{
			let uniforms = uniform! {
				mvp:   mvp,
				color: rgb(0, 0, 0),
			};

			target.draw(&self.vertices, &self.borders, &self.program, &uniforms, &DrawParameters {
				depth_test:  IfLessOrEqual,
				depth_write: true,

				line_width: Some(2.0),

				.. Default::default() }).unwrap();
		}
	}
}

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
}

implement_vertex!(Vertex, position);
