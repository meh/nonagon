use std::borrow::Cow;

use ffmpeg::frame;

use glium::texture::{Texture2dDataSource, RawImage2d, SrgbTexture2d};
use glium::texture::ClientFormat::U8U8U8;
use glium::texture::MipmapsOption::NoMipmap;
use glium::{Program, Display, VertexBuffer, Surface};
use glium::index::PrimitiveType::TriangleStrip;
use glium::index::NoIndices;

use renderer::{Render, Support};

#[derive(Copy, Clone, Debug)]
struct Vertex {
	position: [f32; 2],
	texture:  [f32; 2],
}

implement_vertex!(Vertex, position, texture);

struct Texture<'a> {
	data: &'a [u8],

	width:  u32,
	height: u32,
}

impl<'a> Texture<'a> {
	pub fn new(display: &Display, frame: &frame::Video) -> SrgbTexture2d {
		SrgbTexture2d::with_mipmaps(display, Texture {
			data: frame.data()[0],

			width:  frame.width(),
			height: frame.height(),
		}, NoMipmap).unwrap()
	}
}

impl<'a> Texture2dDataSource<'a> for Texture<'a> {
	type Data = u8;

	fn into_raw(self) -> RawImage2d<'a, u8> {
		RawImage2d {
			data:   Cow::Borrowed(self.data),
			width:  self.width,
			height: self.height,
			format: U8U8U8,
		}
	}
}

pub struct Video<'a> {
	display: &'a Display,

	program:  Program,
	vertices: VertexBuffer<Vertex>,
}

impl<'a> Video<'a> {
	pub fn new<'b>(display: &'b Display) -> Video<'b> {
		Video {
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
						uniform float alpha;

						varying vec2 v_texture;

						void main() {
							gl_FragColor   = texture2D(tex, v_texture);
							gl_FragColor.a = alpha;
						}
					",
				},
			).unwrap(),

			vertices: VertexBuffer::new(display, &[
				Vertex { position: [-1.0,  1.0], texture: [0.0, 0.0] },
				Vertex { position: [ 1.0,  1.0], texture: [1.0, 0.0] },
				Vertex { position: [-1.0, -1.0], texture: [0.0, 1.0] },
				Vertex { position: [ 1.0, -1.0], texture: [1.0, 1.0] },
			]).unwrap(),
		}
	}
}

impl<'a> Render<frame::Video> for Video<'a> {
	fn render<T: Surface>(&self, target: &mut T, support: &Support, frame: &Self::State) {
		let texture = Texture::new(self.display, frame);

		let uniforms = uniform! {
			alpha: 0.8,
			tex:  support.config().texture().filtering().background().sampled(&texture),
		};

		target.clear_color(1.0, 1.0, 1.0, 1.0);
		target.draw(&self.vertices, &NoIndices(TriangleStrip), &self.program, &uniforms, &Default::default()).unwrap();
	}
}
