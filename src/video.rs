use std::mem;
use std::borrow::Cow;
use std::default::Default;

use ffmpeg::{Error, frame, time};

use glium::texture::{Texture2dDataSource, RawImage2d, SrgbTexture2d};
use glium::texture::ClientFormat::U8U8U8;
use glium::{Program, Display, VertexBuffer, IndexBuffer, Surface};
use glium::index::TriangleStrip;

use ::source::video as source;

pub struct Video<'a> {
	source: &'a source::Video,
	done:   bool,

	time:    i64,
	current: frame::Video,
	next:    frame::Video,

	display:  &'a Display,
	program:  Program,
	vertices: VertexBuffer<Vertex>,
	indices:  IndexBuffer,
}

impl<'a> Video<'a> {
	pub fn new<'b>(display: &'b Display, source: &'b source::Video) -> Result<Video<'b>, Error> {
		let program = program!(display,
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
			time:    time::relative(),
			current: try!(frame(&source)),
			next:    try!(frame(&source)),

			source: source,
			done:   false,

			display:  display,
			program:  program,
			vertices: vertices,
			indices:  indices,
		})
	}

	pub fn is_done(&self) -> bool {
		self.done
	}

	pub fn frame(&self) -> &frame::Video {
		&self.current
	}

	pub fn sync(&mut self) {
		let base: f64 = self.source.time_base().into();
		let time: f64 = (time::relative() - self.time) as f64 / 1_000_000.0;
		let pts:  f64 = self.next.timestamp().unwrap_or(0) as f64 * base;

		if time > pts {
			match try_frame(&self.source) {
				Some(Ok(frame)) => {
					mem::swap(&mut self.current, &mut self.next);
					self.next = frame;
				},

				Some(Err(Error::Eof)) =>
					self.done = true,

				Some(Err(error)) =>
					debug!("{:?}", error),

				_ => ()
			}
		}
	}

	pub fn texture(&self) -> SrgbTexture2d {
		SrgbTexture2d::new(self.display, Texture {
			data: self.frame().picture().data()[0],

			width:  self.frame().picture().width(),
			height: self.frame().picture().height(),
		})
	}

	pub fn draw<T: Surface>(&self, target: &mut T) {
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

fn frame(source: &source::Video) -> Result<frame::Video, Error> {
	loop {
		match source.recv() {
			Ok(source::Data::Frame(frame)) =>
				return Ok(frame),

			Ok(source::Data::Error(error)) => {
				debug!("{:?}", error);
				continue;
			},

			Ok(source::Data::End) =>
				return Err(Error::Eof),

			_ =>
				return Err(Error::Bug)
		}
	}
}

fn try_frame(source: &source::Video) -> Option<Result<frame::Video, Error>> {
	match source.try_recv() {
		Ok(source::Data::Frame(frame)) =>
			Some(Ok(frame)),

		Ok(source::Data::Error(error)) =>
			Some(Err(error)),

		Ok(source::Data::End) =>
			Some(Err(Error::Eof)),

		_ =>
			None
	}
}
