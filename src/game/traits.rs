use game::Support;

pub trait Update {
	fn update(&mut self, support: &Support);
}

pub trait Alive {
	fn alive(&self, support: &Support) -> bool;
}

pub trait CanDamage<T, U> {
	fn can_damage(a: &T, b: &U) -> bool;
}
