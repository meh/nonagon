use ffmpeg::Rational;

pub trait Aspect {
	fn is_vertical(&self) -> bool;
	fn is_horizontal(&self) -> bool;

	fn height(&self) -> u32;
	fn width(&self) -> u32;
}

impl Aspect for Rational {
	fn is_vertical(&self) -> bool {
		self.0 < self.1
	}

	fn is_horizontal(&self) -> bool {
		self.0 > self.1
	}

	fn height(&self) -> u32 {
		match self {
			&Rational(3, 4)  => 640,
			&Rational(16, 9) => 640,

			_ => unreachable!()
		}
	}

	fn width(&self) -> u32 {
		match self {
			&Rational(3, 4)  => 480,
			&Rational(16, 9) => 360,

			_ => unreachable!()
		}
	}
}
