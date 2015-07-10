use glium::{Program, Display, VertexBuffer, IndexBuffer, Surface, DrawParameters};
use glium::DepthTest::{IfLess, IfLessOrEqual};
use glium::BlendingFunction::Addition;
use glium::LinearBlendingFactor::{SourceAlpha, OneMinusSourceAlpha};
use glium::BackfaceCullingMode::CullClockWise;
use glium::index::NoIndices;
use glium::index::PrimitiveType::{TrianglesList, LinesList};

use util::Fill;
use game::{self, ship};
use renderer::Support;

struct Shape {
	faces:   VertexBuffer<Vertex>,
	borders: IndexBuffer<u16>,
}

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
	texture:  [f32; 2],
}

implement_vertex!(Vertex, position, texture);

pub struct Ship<'a> {
	display: &'a Display,

	with_color:   Program,
	with_texture: Program,

	cube:        Shape,
	tetrahedron: Shape,
	octahedron:  Shape,
}

impl<'a> Ship<'a>{
	pub fn new<'b>(display: &'b Display) -> Ship<'b> {
		Ship {
			display: display,

			with_color: program!(display,
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

			with_texture: program!(display,
				110 => {
					vertex: "
						#version 110

						attribute vec3 position;
						attribute vec2 texture;

						uniform mat4 mvp;

						varying vec2 v_texture;

						void main() {
							gl_Position = mvp * vec4(position, 1.0);
							v_texture   = texture;
						}
					",

					fragment: "
						#version 110

						uniform sampler2D tex;

						varying vec2 v_texture;

						void main() {
							gl_FragColor = texture2D(tex, v_texture);
						}
					",
				}).unwrap(),

			cube: Shape {
				faces: {
					#[inline(always)]
					fn coordinates(face: u8, u: f32, v: f32) -> [f32; 2] {
						match face {
							1 => [u / 3.0,                   v / 2.0 + 1.0 / 2.0],
							2 => [u / 3.0 + 1.0 / 3.0,       v / 2.0 + 1.0 / 2.0],
							3 => [u / 3.0 + 1.0 / 3.0 * 2.0, v / 2.0 + 1.0 / 2.0],

							4 => [u / 3.0,                   v / 2.0],
							5 => [u / 3.0 + 1.0 / 3.0,       v / 2.0],
							6 => [u / 3.0 + 1.0 / 3.0 * 2.0, v / 2.0],

							_ => unreachable!()
						}
					}

					VertexBuffer::new(display, vec![
						// front
						Vertex { position: [-1.0, -1.0,  1.0], texture: coordinates(1, 0.0, 0.0) },
						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(1, 1.0, 0.0) },
						Vertex { position: [ 1.0,  1.0,  1.0], texture: coordinates(1, 1.0, 1.0) },

						Vertex { position: [ 1.0,  1.0,  1.0], texture: coordinates(1, 1.0, 1.0) },
						Vertex { position: [-1.0,  1.0,  1.0], texture: coordinates(1, 0.0, 1.0) },
						Vertex { position: [-1.0, -1.0,  1.0], texture: coordinates(1, 0.0, 0.0) },

						// right
						Vertex { position: [ 1.0,  1.0, -1.0], texture: coordinates(2, 1.0, 1.0) },
						Vertex { position: [ 1.0,  1.0,  1.0], texture: coordinates(2, 0.0, 1.0) },
						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(2, 0.0, 0.0) },

						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(2, 0.0, 0.0) },
						Vertex { position: [ 1.0, -1.0, -1.0], texture: coordinates(2, 1.0, 0.0) },
						Vertex { position: [ 1.0,  1.0, -1.0], texture: coordinates(2, 1.0, 1.0) },

						// back
						Vertex { position: [-1.0, -1.0, -1.0], texture: coordinates(3, 1.0, 0.0) },
						Vertex { position: [-1.0,  1.0, -1.0], texture: coordinates(3, 1.0, 1.0) },
						Vertex { position: [ 1.0, -1.0, -1.0], texture: coordinates(3, 0.0, 0.0) },

						Vertex { position: [ 1.0,  1.0, -1.0], texture: coordinates(3, 0.0, 1.0) },
						Vertex { position: [ 1.0, -1.0, -1.0], texture: coordinates(3, 0.0, 0.0) },
						Vertex { position: [-1.0,  1.0, -1.0], texture: coordinates(3, 1.0, 1.0) },

						// left
						Vertex { position: [-1.0, -1.0,  1.0], texture: coordinates(4, 1.0, 0.0) },
						Vertex { position: [-1.0,  1.0,  1.0], texture: coordinates(4, 1.0, 1.0) },
						Vertex { position: [-1.0, -1.0, -1.0], texture: coordinates(4, 0.0, 0.0) },

						Vertex { position: [-1.0, -1.0, -1.0], texture: coordinates(4, 0.0, 0.0) },
						Vertex { position: [-1.0,  1.0,  1.0], texture: coordinates(4, 1.0, 1.0) },
						Vertex { position: [-1.0,  1.0, -1.0], texture: coordinates(4, 0.0, 1.0) },

						// top
						Vertex { position: [-1.0,  1.0,  1.0], texture: coordinates(5, 0.0, 0.0) },
						Vertex { position: [ 1.0,  1.0,  1.0], texture: coordinates(5, 1.0, 0.0) },
						Vertex { position: [ 1.0,  1.0, -1.0], texture: coordinates(5, 1.0, 1.0) },

						Vertex { position: [-1.0,  1.0,  1.0], texture: coordinates(5, 0.0, 0.0) },
						Vertex { position: [ 1.0,  1.0, -1.0], texture: coordinates(5, 1.0, 1.0) },
						Vertex { position: [-1.0,  1.0, -1.0], texture: coordinates(5, 0.0, 1.0) },

						// bottom
						Vertex { position: [-1.0, -1.0,  1.0], texture: coordinates(6, 0.0, 1.0) },
						Vertex { position: [-1.0, -1.0, -1.0], texture: coordinates(6, 0.0, 0.0) },
						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(6, 1.0, 1.0) },

						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(6, 1.0, 1.0) },
						Vertex { position: [-1.0, -1.0, -1.0], texture: coordinates(6, 0.0, 0.0) },
						Vertex { position: [ 1.0, -1.0, -1.0], texture: coordinates(6, 1.0, 0.0) },
					])
				},

				borders: IndexBuffer::new(display, LinesList, vec![
					// front
					0, 1,
					1, 2,
					2, 4,
					4, 0,

					// back
					12, 10,
					10,  6,
				 	 6, 13,
					13, 12,

					// left
					0, 12,
					4, 13,

					// right
					1, 10,
					2,  6,
				])
			},

			tetrahedron: Shape {
				faces: {
					#[inline(always)]
					fn coordinates(face: u8, u: f32, v: f32) -> [f32; 2] {
						match face {
							1 => [u / 2.0,             v / 2.0 + 1.0 / 2.0],
							2 => [u / 2.0 + 1.0 / 2.0, v / 2.0 + 1.0 / 2.0],

							3 => [u / 2.0,             v / 2.0],
							4 => [u / 2.0 + 1.0 / 2.0, v / 2.0],

							_ => unreachable!()
						}
					}

					VertexBuffer::new(display, vec![
						// front
						Vertex { position: [ 0.0,  1.0,  0.0], texture: coordinates(1, 0.5, 1.0) },
						Vertex { position: [-1.0, -1.0,  1.0], texture: coordinates(1, 0.0, 0.0) },
						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(1, 1.0, 0.0) },

						// left
						Vertex { position: [ 0.0,  1.0,  0.0], texture: coordinates(2, 0.5, 1.0) },
						Vertex { position: [ 0.0, -1.0, -1.0], texture: coordinates(2, 0.0, 0.0) },
						Vertex { position: [-1.0, -1.0,  1.0], texture: coordinates(2, 1.0, 0.0) },

						// right
						Vertex { position: [ 0.0,  1.0,  0.0], texture: coordinates(3, 0.5, 1.0) },
						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(3, 0.0, 0.0) },
						Vertex { position: [ 0.0, -1.0, -1.0], texture: coordinates(3, 1.0, 0.0) },

						// bottom
						Vertex { position: [-1.0, -1.0,  1.0], texture: coordinates(4, 0.5, 1.0) },
						Vertex { position: [ 0.0, -1.0, -1.0], texture: coordinates(4, 0.0, 0.0) },
						Vertex { position: [ 1.0, -1.0,  1.0], texture: coordinates(4, 1.0, 0.0) },
					])
				},

				borders: IndexBuffer::new(display, LinesList, vec![
					// front
					0, 1,

					// left
					3, 4,

					// right
					2, 3,

					// bottom left
					4, 5,

					// bottom right
					1, 2,

					// bottom back
					7, 8,
				])
			},

			octahedron: Shape {
				faces: {
					#[inline(always)]
					fn coordinates(face: u8, u: f32, v: f32) -> [f32; 2] {
						match face {
							1 => [u, v],
							2 => [u, v],
							3 => [u, v],
							4 => [u, v],

							5 => [u, v],
							6 => [u, v],
							7 => [u, v],
							8 => [u, v],

							_ => unreachable!()
						}
					}

					VertexBuffer::new(display, vec![
						// top front
						Vertex { position: [-1.0,  0.0,  1.0], texture: coordinates(1, 0.0, 0.0) },
						Vertex { position: [ 1.0,  0.0,  1.0], texture: coordinates(1, 0.0, 0.0) },
						Vertex { position: [ 0.0,  1.0,  0.0], texture: coordinates(1, 0.0, 0.0) },

						// top right
						Vertex { position: [ 1.0,  0.0,  1.0], texture: coordinates(2, 0.0, 0.0) },
						Vertex { position: [ 1.0,  0.0, -1.0], texture: coordinates(2, 0.0, 0.0) },
						Vertex { position: [ 0.0,  1.0,  0.0], texture: coordinates(2, 0.0, 0.0) },

						// top back
						Vertex { position: [ 1.0,  0.0, -1.0], texture: coordinates(3, 0.0, 0.0) },
						Vertex { position: [-1.0,  0.0, -1.0], texture: coordinates(3, 0.0, 0.0) },
						Vertex { position: [ 0.0,  1.0,  0.0], texture: coordinates(3, 0.0, 0.0) },

						// top left
						Vertex { position: [-1.0,  0.0, -1.0], texture: coordinates(4, 0.0, 0.0) },
						Vertex { position: [-1.0,  0.0,  1.0], texture: coordinates(4, 0.0, 0.0) },
						Vertex { position: [ 0.0,  1.0,  0.0], texture: coordinates(4, 0.0, 0.0) },

						// bottom front
						Vertex { position: [ 1.0,  0.0,  1.0], texture: coordinates(5, 0.0, 0.0) },
						Vertex { position: [-1.0,  0.0,  1.0], texture: coordinates(5, 0.0, 0.0) },
						Vertex { position: [ 0.0, -1.0,  0.0], texture: coordinates(5, 0.0, 0.0) },

						// bottom right
						Vertex { position: [ 1.0,  0.0, -1.0], texture: coordinates(6, 0.0, 0.0) },
						Vertex { position: [ 1.0,  0.0,  1.0], texture: coordinates(6, 0.0, 0.0) },
						Vertex { position: [ 0.0, -1.0,  0.0], texture: coordinates(6, 0.0, 0.0) },

						// bottom back
						Vertex { position: [-1.0,  0.0, -1.0], texture: coordinates(7, 0.0, 0.0) },
						Vertex { position: [ 1.0,  0.0, -1.0], texture: coordinates(7, 0.0, 0.0) },
						Vertex { position: [ 0.0, -1.0,  0.0], texture: coordinates(7, 0.0, 0.0) },

						// bottom left
						Vertex { position: [-1.0,  0.0,  1.0], texture: coordinates(8, 0.0, 0.0) },
						Vertex { position: [-1.0,  0.0, -1.0], texture: coordinates(8, 0.0, 0.0) },
						Vertex { position: [ 0.0, -1.0,  0.0], texture: coordinates(8, 0.0, 0.0) },
					])
				},

				borders: IndexBuffer::new(display, LinesList, vec![
					// front middle
					0, 1,

					// top front right
					1, 2,

					// top front left
					0, 2,

					// right middle
					3, 4,

					// top back left
					4, 5,

					// back middle
					6, 7,

					// top back right
					7, 8,

					// left middle
					9, 10,

					// bottom front left
					13, 14,

					// bottom back left
					14, 15,

					// bottom front right
					16, 17,

					// bottom back right
					17, 18,
				])
			},
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, support: &Support, state: &game::Ship) {
		let (faces, borders) = match state.shape {
			ship::Shape::Cube =>
				(&self.cube.faces, &self.cube.borders),

			ship::Shape::Tetrahedron =>
				(&self.tetrahedron.faces, &self.tetrahedron.borders),

			ship::Shape::Octahedron =>
				(&self.octahedron.faces, &self.octahedron.borders),
		};

		let mvp = support.scene().to_mat() *
			support.scene().position(state.position) *
			support.scene().orientation(state.orientation) *
			support.scene().scale(12.5 * state.scale) *
			support.scene().depth(state.position);

		// draw the faces
		match state.face {
			Fill::Color(color) => {
				let uniforms = uniform! {
					mvp:   mvp,
					color: color,
				};

				target.draw(faces, &NoIndices(TrianglesList), &self.with_color, &uniforms, &DrawParameters {
					backface_culling: CullClockWise,

					blending_function: Some(Addition {
						source:      SourceAlpha,
						destination: OneMinusSourceAlpha
					}),

					depth_test:  IfLess,
					depth_write: true,

					.. Default::default() }).unwrap();
			},

			Fill::Texture(ref path) => {
				let texture  = support.assets().texture(path);
				let uniforms = uniform! {
					mvp: mvp,
					tex: &*texture,
				};

				target.draw(faces, &NoIndices(TrianglesList), &self.with_texture, &uniforms, &DrawParameters {
					backface_culling: CullClockWise,

					blending_function: Some(Addition {
						source:      SourceAlpha,
						destination: OneMinusSourceAlpha
					}),

					depth_test:  IfLess,
					depth_write: true,

					.. Default::default() }).unwrap();
			}
		}

		// draw the borders
		match state.border {
			Some(Fill::Color(color)) => {
				let uniforms = uniform! {
					mvp:   mvp,
					color: color,
				};

				target.draw(faces, borders, &self.with_color, &uniforms, &DrawParameters {
					depth_test:  IfLessOrEqual,
					depth_write: true,

					line_width: Some(2.0),

					.. Default::default() }).unwrap();
			},

			Some(Fill::Texture(ref path)) => {
				let texture  = support.assets().texture(path);
				let uniforms = uniform! {
					mvp: mvp,
					tex: &*texture,
				};

				target.draw(faces, borders, &self.with_texture, &uniforms, &DrawParameters {
					depth_test:  IfLessOrEqual,
					depth_write: true,

					line_width: Some(2.0),

					.. Default::default() }).unwrap();
			},

			_ => ()
		}
	}
}
