use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::cell::RefCell;
use std::rc::Rc;

use glium::Display;
use glium::texture::Texture2d;
use glium::texture::MipmapsOption::NoMipmap;

use image;

pub struct Assets<'a> {
	display: &'a Display,

	textures: RefCell<HashMap<PathBuf, Rc<Texture2d>>>,
}

impl<'a> Assets<'a> {
	pub fn new(display: &Display) -> Assets {
		Assets {
			display: display,

			textures: RefCell::new(HashMap::new()),
		}
	}

	pub fn texture(&self, path: &Path) -> Rc<Texture2d> {
		if let Some(tex) = self.textures.borrow().get(path) {
			return tex.clone();
		}

		let img = image::open(path).unwrap();
		let tex = Texture2d::with_mipmaps(self.display, img, NoMipmap).unwrap();

		self.textures.borrow_mut().insert(path.to_owned(), Rc::new(tex));
		self.textures.borrow().get(path).unwrap().clone()
	}
}
