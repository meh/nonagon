use std::ops::Deref;

pub struct Ring<T> {
	buffer:   Vec<T>,
	size:     usize,
	position: usize,
}

impl<T> Ring<T> {
	pub fn new(size: usize) -> Self {
		Ring {
			buffer:   Vec::with_capacity(size),
			size:     size,
			position: 0,
		}
	}

	pub fn push(&mut self, value: T) {
		if self.buffer.len() >= self.size {
			self.buffer[self.position] = value;
		}
		else {
			self.buffer.insert(self.position, value);
		}

		self.position += 1;

		if self.position >= self.size {
			self.position = 0;
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.buffer.is_empty() {
			return None;
		}

		let result = if self.buffer.len() < self.size {
			self.pop()
		}
		else {
			Some(self.buffer.remove(self.position))
		};

		if self.position != 0 {
			self.position -= 1;
		}
		else {
			self.position = self.buffer.len() - 1;
		}

		result
	}
}

impl<T> Deref for Ring<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}
