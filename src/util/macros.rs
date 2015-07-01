macro_rules! ret {
	($body:expr) => (
		match { $body } {
			Ok(value) =>
				value,

			Err(..) =>
				return
		}
	);
}
