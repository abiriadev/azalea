use std::collections::HashMap;

use regex::Regex;
use tl::{parse, Node, ParserOptions};

pub struct Template {
	src: String,
}

impl Template {
	pub fn new(src: String) -> Self { Self { src } }

	pub fn render(&self, props: &HashMap<&'static str, String>) -> String {
		// TODO: return Err if parsing fail
		let mut vdom = parse(&self.src, ParserOptions::default()).unwrap();

		for node in vdom.nodes_mut() {
			let bytes = match node {
				Node::Raw(r) => r,
				_ => continue,
			};

			let text = bytes.try_as_utf8_str().unwrap();

			let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();
			let caps = re.captures(text).unwrap();

			let Some(prop) = caps.get(1) else { continue; };

			let Some(value) = props.get(prop.as_str()) else { continue; };

			bytes.set(value.as_str()).unwrap();
		}

		vdom.outer_html()
	}
}
