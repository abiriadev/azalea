use std::collections::HashMap;

pub struct Template {
	src: String,
}

impl Template {
	pub fn new(src: String) -> Self { Self { src } }

	pub fn render(props: HashMap<&'static str, String>) -> String { todo!() }
}
