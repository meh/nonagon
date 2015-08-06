use docopt::ArgvMap;

use toml::{Value, ParserError};

use glium;
use glium::uniforms::{Sampler, SamplerWrapFunction, MagnifySamplerFilter, MinifySamplerFilter};

use settings::Load;

#[derive(Clone, Debug)]
pub struct Video {
	vsync:         bool,
	multisampling: Option<u16>,

	effects: Effects,
	texture: Texture,
}

impl Default for Video {
	fn default() -> Video {
		Video {
			vsync:         true,
			multisampling: None,

			texture: Default::default(),
			effects: Default::default(),
		}
	}
}

impl Load for Video {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = toml.as_table().unwrap();

		if let Some(toml) = toml.get("video") {
			let toml = expect!(toml.as_table(), "`video` must be a table");

			if let Some(value) = toml.get("vsync") {
				self.vsync = expect!(value.as_bool(), "`video.vsync` must be boolean");
			}

			if let Some(value) = toml.get("multisampling") {
				match value {
					&Value::Boolean(false) =>
						self.multisampling = None,

					&Value::Boolean(true) =>
						self.multisampling = Some(2),

					&Value::Integer(value) =>
						self.multisampling = Some(value as u16),

					_ =>
						expect!("`video.multisampling` must be a boolean or integer"),
				}
			}

			if let Some(toml) = toml.get("effects") {
				try!(self.effects.load(args, toml));
			}

			if let Some(toml) = toml.get("texture") {
				try!(self.texture.load(args, toml));
			}
		}

		Ok(())
	}
}

impl Video {
	#[inline(always)]
	pub fn vsync(&self) -> bool {
		self.vsync
	}

	#[inline(always)]
	pub fn multisampling(&self) -> Option<u16> {
		self.multisampling
	}

	#[inline(always)]
	pub fn effects(&self) -> &Effects {
		&self.effects
	}

	#[inline(always)]
	pub fn texture(&self) -> &Texture {
		&self.texture
	}
}

#[derive(Clone, Default, Debug)]
pub struct Effects {
	bullet: Bullet,
}

impl Load for Effects {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`video.effects` must be a table");

		if let Some(toml) = toml.get("bullet") {
			try!(self.bullet.load(args, toml));
		}

		Ok(())
	}
}

impl Effects {
	pub fn bullet(&self) -> &Bullet {
		&self.bullet
	}
}

#[derive(Clone, Default, Debug)]
pub struct Bullet {
	plasma: Plasma,
}

impl Load for Bullet {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`video.effects.bullet` must be a table");

		if let Some(toml) = toml.get("plasma") {
			try!(self.plasma.load(args, toml));
		}

		Ok(())
	}
}

impl Bullet {
	#[inline(always)]
	pub fn plasma(&self) -> &Plasma {
		&self.plasma
	}
}

#[derive(Clone, Debug)]
pub struct Plasma {
	glow: bool,
}

impl Default for Plasma {
	fn default() -> Plasma {
		Plasma {
			glow: true,
		}
	}
}

impl Load for Plasma {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`video.effects.bullet.plasma` must be a table");

		if let Some(value) = toml.get("glow") {
			self.glow = expect!(value.as_bool(), "`video.effects.bullet.plasma.glow` must be a boolean");
		}

		Ok(())
	}
}

impl Plasma {
	#[inline(always)]
	pub fn glow(&self) -> bool {
		self.glow
	}
}

#[derive(Clone, Default, Debug)]
pub struct Texture {
	filtering: Filtering,
}

impl Load for Texture {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`video.texture` must be a table");

		if let Some(toml) = toml.get("filtering") {
			try!(self.filtering.load(args, toml));
		}

		Ok(())
	}
}

impl Texture {
	#[inline(always)]
	pub fn filtering(&self) -> &Filtering {
		&self.filtering
	}
}

#[derive(Clone, Default, Debug)]
pub struct Filtering {
	background: Filter,
	ship:       Filter,
}

impl Load for Filtering {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`video.texture.filtering` must be a table");

		if let Some(toml) = toml.get("background") {
			try!(self.background.load(args, toml));
		}

		if let Some(toml) = toml.get("ship") {
			try!(self.ship.load(args, toml));
		}

		Ok(())
	}
}

impl Filtering {
	#[inline(always)]
	pub fn background(&self) -> &Filter {
		&self.background
	}

	#[inline(always)]
	pub fn ship(&self) -> &Filter {
		&self.ship
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Filter {
	wrap:       Option<SamplerWrapFunction>,
	magnify:    Option<MagnifySamplerFilter>,
	minify:     Option<MinifySamplerFilter>,
	anisotropy: Option<u16>,
}

impl Default for Filter {
	fn default() -> Filter {
		Filter {
			wrap:       None,
			magnify:    Some(MagnifySamplerFilter::Linear),
			minify:     Some(MinifySamplerFilter::Linear),
			anisotropy: Some(16),
		}
	}
}

impl Load for Filter {
	fn load(&mut self, args: &ArgvMap, toml: &Value) -> Result<(), ParserError> {
		let toml = expect!(toml.as_table(), "`video.texture.filtering.*` must be a table");

		if let Some(value) = toml.get("wrap") {
			match value {
				&Value::Boolean(true) =>
					(),

				&Value::Boolean(false) =>
					self.wrap = None,

				&Value::String(ref string) =>
					self.wrap = Some(match string.as_ref() {
						"repeat" =>
							SamplerWrapFunction::Repeat,

						"mirror" =>
							SamplerWrapFunction::Mirror,

						"clamp" =>
							SamplerWrapFunction::Clamp,

						"mirror-clamp" =>
							SamplerWrapFunction::MirrorClamp,

						_ =>
							expect!("`video.texture.filtering.*.wrap` must be 'repeat' or 'mirror' or 'clamp' or 'mirror-clamp'"),
					}),

				_ =>
					expect!("`video.texture.filtering.*.wrap` must be a boolean or a string"),
			}
		}

		if let Some(value) = toml.get("minify") {
			match value {
				&Value::Boolean(true) =>
					self.minify = Some(MinifySamplerFilter::Linear),

				&Value::Boolean(false) =>
					self.minify = None,

				&Value::String(ref string) =>
					self.minify = Some(match string.as_ref() {
						"nearest" =>
							MinifySamplerFilter::Nearest,

						"linear" =>
							MinifySamplerFilter::Linear,

						"nearest-mipmap-nearest" =>
							MinifySamplerFilter::NearestMipmapNearest,

						"linear-mimpmap-nearest" =>
							MinifySamplerFilter::LinearMipmapNearest,

						"nearest-mipmap-linear" =>
							MinifySamplerFilter::NearestMipmapLinear,

						"linear-mipmap-linear" =>
							MinifySamplerFilter::LinearMipmapLinear,

						_ =>
							expect!("`video.texture.filtering.*.minify` must be 'nearest' or 'linear' or 'nearest-mipmap-nearest' or 'linear-mipmap-nearest' or 'nearest-mipmap-linear' or 'linear-mipmap-linear'"),
					}),

				_ =>
					expect!("`video.texture.filtering.*.minify` must be a boolean or a string"),
			}
		}

		if let Some(value) = toml.get("magnify") {
			match value {
				&Value::Boolean(true) =>
					self.magnify = Some(MagnifySamplerFilter::Linear),

				&Value::Boolean(false) =>
					self.magnify = None,

				&Value::String(ref string) =>
					self.magnify = Some(match string.as_ref() {
						"nearest" =>
							MagnifySamplerFilter::Nearest,

						"linear" =>
							MagnifySamplerFilter::Linear,

						_ =>
							expect!("`video.texture.filtering.*.magnify` must be 'nearest' or 'linear'"),
					}),

				_ =>
					expect!("`video.texture.filtering.*.magnify` must be a boolean or a string"),
			}
		}

		if let Some(value) = toml.get("anisotropy") {
			match value {
				&Value::Boolean(true) =>
					self.anisotropy = Some(16),

				&Value::Boolean(false) =>
					self.anisotropy = None,

				&Value::Integer(value) =>
					self.anisotropy = Some(value as u16),

				_ =>
					expect!("`video.texture.filtering.*.anisotropy` must be a boolean or a string"),
			}
		}

		Ok(())
	}
}

impl Filter {
	#[inline(always)]
	pub fn wrap(&self) -> Option<SamplerWrapFunction> {
		self.wrap
	}

	#[inline(always)]
	pub fn magnify(&self) -> Option<MagnifySamplerFilter> {
		self.magnify
	}

	#[inline(always)]
	pub fn minify(&self) -> Option<MinifySamplerFilter> {
		self.minify
	}

	#[inline(always)]
	pub fn anisotropy(&self) -> Option<u16> {
		self.anisotropy
	}

	#[inline]
	pub fn sampled<'a, T: glium::Texture>(&self, texture: &'a T) -> Sampler<'a, T> {
		let mut sampled = Sampler::new(texture);

		if let Some(value) = self.wrap() {
			sampled = sampled.wrap_function(value);
		}

		if let Some(value) = self.minify() {
			sampled = sampled.minify_filter(value);
		}

		if let Some(value) = self.magnify() {
			sampled = sampled.magnify_filter(value);
		}

		if let Some(value) = self.anisotropy() {
			sampled = sampled.anisotropy(value);
		}

		sampled
	}
}
