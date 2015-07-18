use glium::Surface;
use super::Support;

pub trait Render<T> {
	fn render<S: Surface>(&self, target: &mut S, support: &Support, state: &T);
}
