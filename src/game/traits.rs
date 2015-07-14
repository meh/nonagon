use nc::bounding_volume::{HasAABB, AABB};
use game::Support;

pub trait Update {
	fn update(&mut self, support: &Support);
}

pub trait Geom<P, M> {
	type Shape: HasAABB<P, M>;

	fn geom(&self) -> Self::Shape;
}

impl<P, M, S: HasAABB<P, M>> HasAABB<P, M> for Geom<P, M, Shape=S> {
	fn aabb(&self, m: &M) -> AABB<P> {
		self.geom().aabb(m)
	}
}

pub trait CanDamage<T, U> {
	fn can_damage(a: &T, b: &U) -> bool;
}
