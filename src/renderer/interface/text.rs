use glium::{Display, Surface, Program, VertexBuffer, DrawParameters};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::BlendingFunction::Addition;
use glium::LinearBlendingFactor::{SourceAlpha, OneMinusSourceAlpha};
use glium::index::NoIndices;
use glium::index::PrimitiveType::TrianglesList;

use na::{self, Mat4, Iso3, Vec3};

use renderer::{Render, Support};
use renderer::support::Scene;
use renderer::interface::Font;
use util::{Color, Aspect};

#[derive(Copy, Clone, Debug)]
struct Vertex {
	position: [f64; 2],
	texture:  [f64; 2],
}

implement_vertex!(Vertex, position, texture);

pub struct Text<'a> {
	display: &'a Display,
	program: Program,
}

impl<'a> Text<'a> {
	pub fn new(display: &Display) -> Text {
		Text {
			display: display,

			program: program!(display,
				100 => {
					vertex: "
						#version 100

						precision lowp float;

						attribute vec2 position;
						attribute vec2 texture;

						uniform mat4 mvp;

						varying vec2 v_texture;

						void main() {
							gl_Position = mvp * vec4(position, 0.0, 1.0);
							v_texture   = texture;
						}
					",

					fragment: "
						#version 100

						precision lowp float;

						uniform sampler2D font;
						uniform vec4      color;

						varying vec2 v_texture;

						void main() {
							vec4 texel = texture2D(font, v_texture);
							vec4 pixel;

							if (texel.rgb == vec3(0.0, 0.0, 0.0)) {
								pixel.rgba = color;
							}
							else {
								pixel.rgba = vec4(1.0, 1.0, 1.0, 0.0);
							}

							gl_FragColor = pixel;
						}
					",
				},
			).unwrap(),
		}
	}
}

impl<'a> Render<(&'a Font<'a>, Color, (u32, u32), u32, &'a str)> for Text<'a> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, &(font, color, (x, y), size, string): &Self::State) {
		let mut vertices = Vec::<Vertex>::new();

		let width = font.bounds().width as f64 / font.bounds().height as f64;

		for (i, c) in string.chars().enumerate() {
			let left  = width * i as f64;
			let right = width * i as f64 + width;

			vertices.extend(&[
				Vertex { position: [left,  0.0], texture: font.coordinates(c, 0.0, 0.0) },
				Vertex { position: [right, 0.0], texture: font.coordinates(c, 1.0, 0.0) },
				Vertex { position: [right, 1.0], texture: font.coordinates(c, 1.0, 1.0) },

				Vertex { position: [right, 1.0], texture: font.coordinates(c, 1.0, 1.0) },
				Vertex { position: [left,  1.0], texture: font.coordinates(c, 0.0, 1.0) },
				Vertex { position: [left,  0.0], texture: font.coordinates(c, 0.0, 0.0) },
			]);
		}

		#[inline(always)]
		fn position(scene: &Scene, x: u32, y: u32) -> Mat4<f32> {
			let x = x as f32 * scene.width() as f32 / scene.aspect().width() as f32;
			let y = y as f32 * scene.height() as f32 / scene.aspect().height() as f32;

			na::to_homogeneous(&Iso3::new(Vec3::new(
				if x > scene.width() as f32 / 2.0 {
					-((scene.width() as f32 / 2.0) - x)
				}
				else {
					x - scene.width() as f32 / 2.0
				},

				-if y > scene.height() as f32 / 2.0 {
					-((scene.height() as f32 / 2.0) - y)
				}
				else {
					y - scene.height() as f32 / 2.0
				},

				-500.0), na::zero()))
		}

		#[inline(always)]
		fn scale(scene: &Scene, font: &Font, factor: u32) -> Mat4<f32> {
			let factor = (factor * font.bounds().height) as f32;

			Mat4::new(factor,    0.0,    0.0, 0.0,
			             0.0, factor,    0.0, 0.0,
			             0.0,    0.0, factor, 0.0,
			             0.0,    0.0,    0.0, 1.0)
		}

		let mvp = support.scene().to_mat() *
			position(support.scene(), x, y) *
			scale(support.scene(), font, size);

		let uniforms = uniform! {
			mvp:   mvp,
			color: color,

			font: font.as_ref().sampled()
				.minify_filter(MinifySamplerFilter::Nearest)
				.magnify_filter(MagnifySamplerFilter::Nearest)
		};

		target.draw(&VertexBuffer::new(self.display, &vertices).unwrap(), &NoIndices(TrianglesList), &self.program, &uniforms, &DrawParameters {
			blending_function: Some(Addition {
				source:      SourceAlpha,
				destination: OneMinusSourceAlpha
			}),

			.. Default::default() }).unwrap();
	}
}
