use glium::{Program, Display, VertexBuffer, Surface, DrawParameters};
use glium::{Depth, Blend};
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
				100 => {
					vertex: "
						#version 100

						precision lowp float;

						attribute vec2 position;

						uniform mat4 mvp;

						varying vec2 v_position;

						void main() {
							gl_Position = mvp * vec4(position, 0.0, 1.0);
							v_position  = position;
						}
					",

					fragment: "
						#version 100

						precision lowp float;

						uniform sampler2D background;
						uniform float width;
						uniform float height;

						uniform float border;
						uniform vec4  color;

						varying vec2 v_position;

						void main() {
							// get the texel from the background video or visualizer
							vec4 texel = texture2D(background,
								vec2(gl_FragCoord.x / width, gl_FragCoord.y / height));

							// invert color
							vec3 pixel = vec3(1.0 - texel.r, 1.0 - texel.g, 1.0 - texel.b);

							// make the smooth borders
							if (v_position.y >= border || v_position.y <= -border) {
								gl_FragColor = vec4(color.rgb, 1.0 - abs(v_position.y));
							}
							else {
								gl_FragColor = vec4(pixel, 1.0);
							}
						}
					",
				}
			).unwrap(),

			vertices: VertexBuffer::new(display, &[
				Vertex { position: [-1.0,  1.0] },
				Vertex { position: [ 0.0,  1.0] },
				Vertex { position: [-1.0, -1.0] },
				Vertex { position: [ 0.0, -1.0] },
			]).unwrap(),
		}
	}
}

impl<'a> Render<game::Ray> for Ray<'a> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &Self::State) {
		match state {
			&game::Ray::Static { position, orientation, width, border, .. } | &game::Ray::Dynamic { position, orientation, width, border, .. } => {
				let mvp = support.scene().to_mat() *
					support.scene().position(position) *
					support.scene().orientation(orientation) *
					support.scene().scale(width) *
					support.scene().transform(1000.0, 1.0, 1.0);

				let uniforms = uniform! {
					mvp: *mvp.as_ref(),

					background: support.as_ref(),
					width:      support.scene().width() as f32,
					height:     support.scene().height() as f32,

					color:  border,
					border: 0.5,
				};

				target.draw(&self.vertices, &NoIndices(TriangleStrip), &self.program, &uniforms, &DrawParameters {
					blend: Blend {
						color: Addition {
							source:      SourceAlpha,
							destination: OneMinusSourceAlpha
						},

						alpha: Addition {
							source:      SourceAlpha,
							destination: OneMinusSourceAlpha
						},

						.. Default::default()
					},

					depth: Depth {
						test:  IfLessOrEqual,
						write: true,

						.. Default::default()
					},

					.. Default::default() }).unwrap();
			}
		}
	}
}
