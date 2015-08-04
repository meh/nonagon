use std::collections::VecDeque;
use std::ops::Deref;

#[derive(Debug)]
pub struct Ring<T> {
	buffer: VecDeque<T>,
	size:   usize,
}

impl<T> Ring<T> {
	pub fn new(size: usize) -> Self {
		Ring {
			buffer: VecDeque::with_capacity(size),
			size:   size,
		}
	}

	pub fn push(&mut self, value: T) {
		if self.buffer.len() >= self.size {
			self.buffer.pop_front();
		}

		self.buffer.push_back(value);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.buffer.pop_front()
	}
}

impl<T> Deref for Ring<T> {
	type Target = VecDeque<T>;

	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}
