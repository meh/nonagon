use glium::{Program, Display, VertexBuffer, Surface, DrawParameters};
use glium::DepthTest::IfLessOrEqual;
use glium::BlendingFunction::Addition;
use glium::LinearBlendingFactor::{SourceAlpha, OneMinusSourceAlpha};
use glium::index::NoIndices;
use glium::index::PrimitiveType::TriangleStrip;

use game::projectile as game;

use renderer::{Render, Support};

#[derive(Copy, Clone, Debug)]
struct Vertex {
	position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Ray<'a> {
	display: &'a Display,

	program:  Program,
	vertices: VertexBuffer<Vertex>,
}

impl<'a> Ray<'a>{
	pub fn new<'b>(display: &'b Display) -> Ray<'b> {
		Ray {
			display: display,

			program: program!(display,
				110 => {
					vertex: "
						#version 110

						attribute vec2 position;

						uniform mat4 mvp;

						varying vec2 v_position;

						void main() {
							gl_Position = mvp * vec4(position, 0.0, 1.0);
							v_position  = position;
						}
					",

					fragment: "
						#version 110

						uniform sampler2D background;
						uniform float width;
						uniform float height;

						uniform float border;

						varying vec2 v_position;

						void main() {
							// get the texel from the background video or visualizer
							vec4 color = texture2D(background,
								vec2(gl_FragCoord.x / width, gl_FragCoord.y / height));

							// invert color and fix alpha
							color.r = 1.0 - color.r;
							color.g = 1.0 - color.g;
							color.b = 1.0 - color.b;
							color.a = 1.0;

							// make the smooth borders
							if (v_position.y >= border || v_position.y <= -border) {
								color.a = 1.0 - abs(v_position.y);
							}

							gl_FragColor = color;
						}
					",
				}
			).unwrap(),

			vertices: VertexBuffer::new(display, vec![
				Vertex { position: [-1.0,  1.0] },
				Vertex { position: [ 1.0,  1.0] },
				Vertex { position: [-1.0, -1.0] },
				Vertex { position: [ 1.0, -1.0] },
			]),
		}
	}
}

impl<'a> Render<game::Ray> for Ray<'a> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &game::Ray) {
		match state {
			&game::Ray::Static { position, orientation, width, .. } | &game::Ray::Dynamic { position, orientation, width, .. } => {
				let mvp = support.scene().to_mat() *
					support.scene().position(position) *
					support.scene().orientation(orientation) *
					support.scene().scale(width) *
					support.scene().transform(1000.0, 1.0, 1.0);

				let uniforms = uniform! {
					mvp: mvp,

					background: support.as_ref(),
					width:      support.scene().width() as f32,
					height:     support.scene().height() as f32,

					border: 0.5,
				};

				target.draw(&self.vertices, &NoIndices(TriangleStrip), &self.program, &uniforms, &DrawParameters {
					blending_function: Some(Addition {
						source:      SourceAlpha,
						destination: OneMinusSourceAlpha
					}),

					depth_test:  IfLessOrEqual,
					depth_write: true,

					.. Default::default() }).unwrap();
			}
		}
	}
}
