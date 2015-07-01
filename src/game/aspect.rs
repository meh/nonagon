use ffmpeg::Rational;

pub trait Aspect {
	fn is_vertical(&self) -> bool;
	fn is_horizontal(&self) -> bool;

	fn height(&self) -> Option<u32>;
	fn width(&self) -> Option<u32>;
}

impl Aspect for Rational {
	fn is_vertical(&self) -> bool {
		self.0 < self.1
	}

	fn is_horizontal(&self) -> bool {
		self.0 > self.1
	}

	fn height(&self) -> Option<u32> {
		match self {
			&Rational(3, 4) =>
				Some(640),

			&Rational(16, 9) =>
				Some(360),

			_ =>
				None
		}
	}

	fn width(&self) -> Option<u32> {
		match self {
			&Rational(3, 4) =>
				Some(480),

			&Rational(16, 9) =>
				Some(640),

			_ =>
				None
		}
	}
}
