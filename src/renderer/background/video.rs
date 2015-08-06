use ffmpeg::frame;

use glium::texture::{SrgbTexture2d};
use glium::{Program, Display, VertexBuffer, Surface};
use glium::buffer::BufferView;
use glium::buffer::BufferMode::Persistent;
use glium::buffer::BufferType::PixelUnpackBuffer;
use glium::index::PrimitiveType::TriangleStrip;
use glium::index::NoIndices;
use glium::texture::SrgbFormat::U8U8U8U8;
use glium::texture::MipmapsOption::NoMipmap;

use renderer::{Support};

#[derive(Copy, Clone, Debug)]
struct Vertex {
	position: [f32; 2],
	texture:  [f32; 2],
}

implement_vertex!(Vertex, position, texture);

pub struct Video<'a> {
	display: &'a Display,

	program:  Program,
	vertices: VertexBuffer<Vertex>,

	timestamp: i64,
	buffer:    Option<BufferView<[(u8, u8, u8, u8)]>>,
	texture:   Option<SrgbTexture2d>,
}

impl<'a> Video<'a> {
	pub fn new<'b>(display: &'b Display) -> Video<'b> {
		Video {
			display: display,

			program: program!(display,
				100 => {
					vertex: "
						#version 100

						precision lowp float;

						attribute vec2 position;
						attribute vec2 texture;

						varying vec2 v_texture;

						void main() {
							gl_Position = vec4(position, 0.0, 1.0);
							v_texture   = texture;
						}
					",

					fragment: "
						#version 100

						precision lowp float;

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

			timestamp: -1,
			buffer:    None,
			texture:   None,
		}
	}

	pub fn render<T: Surface>(&mut self, target: &mut T, support: &Support, frame: &frame::Video) {
		if self.timestamp < frame.timestamp().unwrap() {
			self.timestamp = frame.timestamp().unwrap();

			if self.buffer.is_none() {
				self.buffer = Some(BufferView::empty_array(self.display,
					PixelUnpackBuffer,
					(frame.width() * frame.height()) as usize,
					Persistent).unwrap());

				self.texture = Some(SrgbTexture2d::empty_with_format(self.display,
					U8U8U8U8, NoMipmap, frame.width(), frame.height()).unwrap());
			}

			// write to the buffer
			self.buffer.as_mut().unwrap().write(frame.plane(0));

			// write the buffer to the texture
			self.texture.as_mut().unwrap().main_level()
				.raw_upload_from_pixel_buffer_inverted(self.buffer.as_ref().unwrap().as_slice(),
					0 .. frame.width(), 0 .. frame.height(), 0 .. 1);
		}

		let uniforms = uniform! {
			alpha: 0.8,
			tex:  support.settings().texture().filtering().background().sampled(self.texture.as_ref().unwrap()),
		};

		target.clear_color(1.0, 1.0, 1.0, 1.0);
		target.draw(&self.vertices, &NoIndices(TriangleStrip), &self.program, &uniforms, &Default::default()).unwrap();
	}
}
