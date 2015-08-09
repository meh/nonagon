use settings::analyzer as settings;
use male::onset::Peak;

#[derive(Clone, Debug)]
pub enum Channel {
	Left(f64, Event),
	Right(f64, Event),
	Mono(f64, Event),
}

#[derive(Clone, Debug)]
pub enum Event {
	Beat(Peak<settings::Band>),
}

pub use male::Band;

mod beats;
pub use self::beats::Beats;

mod analyzer;
pub use self::analyzer::Analyzer;
