use std::rc::Rc;

use glium::{Program, Display, VertexBuffer, Surface};
use glium::texture::Texture2d;
use glium::framebuffer::SimpleFrameBuffer;
use glium::index::NoIndices;
use glium::index::PrimitiveType::TriangleStrip;

use ffmpeg::frame;

use renderer::{Render, Support};
use renderer::background::{Video, Visualizer};
use game;

#[derive(Copy, Clone, Debug)]
struct Vertex {
	position: [f32; 2],
	texture:  [f32; 2],
}

implement_vertex!(Vertex, position, texture);

pub struct Background<'a> {
	display: &'a Display,

	video:      Video<'a>,
	visualizer: Visualizer<'a>,

	program:  Program,
	vertices: VertexBuffer<Vertex>,

	texture: Rc<Texture2d>,
}

impl<'a> Background<'a>{
	pub fn new<'b>(display: &'b Display) -> Background<'b> {
		Background {
			display: display,

			video:      Video::new(display),
			visualizer: Visualizer::new(display),

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

						varying vec2 v_texture;

						void main() {
							gl_FragColor = texture2D(tex, v_texture);
						}
					",
				},
			).unwrap(),

			vertices: VertexBuffer::new(display, &[
				Vertex { position: [-1.0,  1.0], texture: [0.0, 1.0] },
				Vertex { position: [ 1.0,  1.0], texture: [1.0, 1.0] },
				Vertex { position: [-1.0, -1.0], texture: [0.0, 0.0] },
				Vertex { position: [ 1.0, -1.0], texture: [1.0, 0.0] },
			]).unwrap(),

			texture: Rc::new(Texture2d::empty(display, 1, 1).unwrap()),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.texture = Rc::new(Texture2d::empty(self.display, width, height).unwrap());
	}

	pub fn render<S: Surface>(&mut self, target: &mut S, support: &Support, state: &game::State, frame: Option<&frame::Video>) {
		// render video or visualizer to the internal texture
		{
			let mut surface = SimpleFrameBuffer::new(self.display, &*self.texture);

			if let Some(frame) = frame {
				self.video.render(&mut surface, support, frame);
			}
			else {
				self.visualizer.render(&mut surface, support, state);
			}
		}

		// draw the internal texture to video
		let uniforms = uniform! {
			tex: &*self.texture,
		};

		target.draw(&self.vertices, &NoIndices(TriangleStrip), &self.program, &uniforms, &Default::default()).unwrap();
	}

	pub fn texture(&self) -> Rc<Texture2d> {
		self.texture.clone()
	}
}
