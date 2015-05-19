use std::borrow::Cow;
use std::default::Default;

use ffmpeg::software::Converter;
use ffmpeg::{Error, frame, format};

use glium::texture::{Texture2dDataSource, RawImage2d, SrgbTexture2d};
use glium::texture::ClientFormat::U8U8U8;
use glium::{Program, Display, VertexBuffer, IndexBuffer, Surface};
use glium::index::TriangleStrip;

use ::source;

pub struct Video<'a> {
	converter: Converter<'a>,
	source:    &'a source::Video,

	display:  &'a Display,
	program:  Program,
	vertices: VertexBuffer<Vertex>,
	indices:  IndexBuffer,
}

impl<'a> Video<'a> {
	pub fn new<'b>(display: &'b Display, source: &'b source::Video) -> Result<Video<'b>, Error> {
		let converter = try!(Converter::new(source.format(), format::Pixel::RGB24, source.width(), source.height()));
		let program   = program!(display,
			140 => {
				vertex: "
					#version 140

					uniform mat4 matrix;

					in vec2 position;
					in vec2 tex_coords;

					out vec2 v_tex_coords;

					void main() {
						gl_Position = matrix * vec4(position, 0.0, 1.0);
						v_tex_coords = tex_coords;
					}
				",

				fragment: "
					#version 140
					uniform sampler2D tex;
					in vec2 v_tex_coords;
					out vec4 f_color;

					void main() {
						f_color = texture(tex, v_tex_coords);
					}
				"
			},

			110 => {
				vertex: "
					#version 110

					uniform mat4 matrix;

					attribute vec2 position;
					attribute vec2 tex_coords;

					varying vec2 v_tex_coords;

					void main() {
						gl_Position = matrix * vec4(position, 0.0, 1.0);
						v_tex_coords = tex_coords;
					}
				",

				fragment: "
					#version 110
					uniform sampler2D tex;
					varying vec2 v_tex_coords;

					void main() {
						gl_FragColor = texture2D(tex, v_tex_coords);
					}
				",
			},

			100 => {
				vertex: "
					#version 100

					uniform lowp mat4 matrix;

					attribute lowp vec2 position;
					attribute lowp vec2 tex_coords;

					varying lowp vec2 v_tex_coords;

					void main() {
						gl_Position = matrix * vec4(position, 0.0, 1.0);
						v_tex_coords = tex_coords;
					}
				",

				fragment: "
					#version 100
					uniform lowp sampler2D tex;
					varying lowp vec2 v_tex_coords;

					void main() {
						gl_FragColor = texture2D(tex, v_tex_coords);
					}
				",
			},
		).unwrap();

		let vertices = VertexBuffer::new(display,
			vec![
				Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 1.0] },
				Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 0.0] },
				Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 0.0] },
				Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 1.0] }
			]);

		let indices = IndexBuffer::new(display, TriangleStrip(vec![1u16, 2, 0, 3]));

		Ok(Video {
			converter: converter,
			source:    source,

			display:  display,
			program:  program,
			vertices: vertices,
			indices:  indices,
		})
	}

	pub fn texture(&self) -> SrgbTexture2d {
		SrgbTexture2d::new(self.display, Texture {
			data: self.converter.data()[0],

			width:  self.converter.width(),
			height: self.converter.height(),
		})
	}

	pub fn draw<T: Surface>(&mut self, target: &mut T) {
		match self.source.try_recv() {
			Ok(source::video::Data::Frame(frame)) =>
				self.converter.convert(&frame.picture()).unwrap(),

			Ok(source::video::Data::Error(error)) =>
				println!("error: ffmpeg: video: {:?}", error),

			_ => ()
		}

		let texture = self.texture();

		let uniforms = uniform! {
			matrix: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0]
			],

			tex: &texture
		};

		target.draw(&self.vertices, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();
	}
}

pub struct Texture<'a> {
	data: &'a [u8],

	width:  u32,
	height: u32,
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

#[derive(Copy, Clone)]
pub struct Vertex {
	position:   [f32; 2],
	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
