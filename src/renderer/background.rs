use glium::{Program, Display, VertexBuffer, Surface};
use glium::texture::SrgbTexture2d;
use glium::index::NoIndices;
use glium::index::PrimitiveType::TriangleStrip;

use renderer::{Render, Support};

#[derive(Copy, Clone, Debug)]
struct Vertex {
	position: [f32; 2],
	texture:  [f32; 2],
}

implement_vertex!(Vertex, position, texture);

pub struct Background<'a> {
	display: &'a Display,

	program:  Program,
	vertices: VertexBuffer<Vertex>,
}

impl<'a> Background<'a>{
	pub fn new<'b>(display: &'b Display) -> Background<'b> {
		Background {
			display: display,

			program: program!(display,
				110 => {
					vertex: "
						#version 110

						attribute vec2 position;
						attribute vec2 texture;

						varying vec2 v_texture;

						void main() {
							gl_Position = vec4(position, 0.0, 1.0);
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
				},
			).unwrap(),

			vertices: VertexBuffer::new(display, vec![
				Vertex { position: [-1.0,  1.0], texture: [0.0, 1.0] },
				Vertex { position: [ 1.0,  1.0], texture: [1.0, 1.0] },
				Vertex { position: [-1.0, -1.0], texture: [0.0, 0.0] },
				Vertex { position: [ 1.0, -1.0], texture: [1.0, 0.0] },
			]),
		}
	}
}

impl<'a> Render<SrgbTexture2d> for Background<'a> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &SrgbTexture2d) {
		let uniforms = uniform! {
			tex: state,
		};

		target.draw(&self.vertices, &NoIndices(TriangleStrip), &self.program, &uniforms, &Default::default()).unwrap();
	}
}
