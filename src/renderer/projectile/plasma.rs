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

pub struct Plasma<'a> {
	display: &'a Display,

	program:  Program,
	vertices: VertexBuffer<Vertex>,
}

impl<'a> Plasma<'a>{
	pub fn new<'b>(display: &'b Display) -> Plasma<'b> {
		Plasma {
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
							vec4 texel = texture2D(background,
								vec2(gl_FragCoord.x / width, gl_FragCoord.y / height));

							// invert color and fix alpha
							vec4 color = vec4(1.0 - texel.r, 1.0 - texel.g, 1.0 - texel.b, 1.0);

							// if gray it means we're invisible, make it black
							if (color.r == color.g && color.g == color.b) {
								if (color.r >= 0.48 && color.r <= 0.52) {
									color.rgb = vec3(0.0, 0.0, 0.0);
								}
							}

							// make the circle
							float dist = 1.0 - sqrt(v_position.x * v_position.x + v_position.y * v_position.y);
							float t    = 0.0;

							if (dist > border) {
								t = 1.0;
							}
							else if (dist > 0.0) {
								t = dist / border;
							}

							gl_FragColor = mix(vec4(color.rgb, 0.0), color, t);
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

impl<'a> Render<game::Plasma> for Plasma<'a> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &Self::State) {
		match state {
			&game::Plasma::Static { position, radius, .. } | &game::Plasma::Dynamic { position, radius, .. } => {
				let mvp = support.scene().to_mat() *
					support.scene().position(position) *
					support.scene().scale(radius * 2.0) *
					support.scene().depth(position);

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
