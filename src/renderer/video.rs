use std::borrow::Cow;
use std::default::Default;

use ffmpeg::frame;

use glium::texture::{Texture2dDataSource, RawImage2d, SrgbTexture2d};
use glium::texture::ClientFormat::U8U8U8;
use glium::texture::MipmapsOption::NoMipmap;
use glium::{Program, Display, VertexBuffer, IndexBuffer, Surface};
use glium::index::PrimitiveType::TriangleStrip;

use renderer::Support;

pub struct Video<'a> {
	display: &'a Display,

	program:  Program,
	vertices: VertexBuffer<Vertex>,
	indices:  IndexBuffer<u16>,
}

impl<'a> Video<'a> {
	pub fn new<'b>(display: &'b Display) -> Video<'b> {
		let program = program!(display,
			110 => {
				vertex: "
					#version 110

					attribute vec2 position;
					attribute vec2 texture;

					varying vec2 v_texture;

					void main() {
						gl_Position = vec4(position, 0.0, 1.0);
						v_texture = texture;
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
		).unwrap();

		let vertices = VertexBuffer::new(display, vec![
			Vertex { position: [-1.0, -1.0], texture: [0.0, 1.0] },
			Vertex { position: [-1.0,  1.0], texture: [0.0, 0.0] },
			Vertex { position: [ 1.0,  1.0], texture: [1.0, 0.0] },
			Vertex { position: [ 1.0, -1.0], texture: [1.0, 1.0] },
		]);

		let indices = IndexBuffer::new(display, TriangleStrip, vec![1, 2, 0, 3]);

		Video {
			display: display,

			program:  program,
			vertices: vertices,
			indices:  indices,
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, support: &Support, frame: &frame::Video) {
		let texture = Texture::new(self.display, frame);

		let uniforms = uniform! {
			tex: &texture
		};

		target.draw(&self.vertices, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();
	}
}

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 2],
	texture:  [f32; 2],
}

implement_vertex!(Vertex, position, texture);

pub struct Texture<'a> {
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
		}, NoMipmap)
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
