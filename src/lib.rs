use std::{collections::HashMap, sync::OnceLock};

use regex::Regex;
use tl::{parse, Node, ParserOptions};

static PROP_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct Template {
	src: String,
}

impl Template {
	pub fn new(src: String) -> Self { Self { src } }

	pub fn render(&self, props: &HashMap<&'static str, String>) -> String {
		// TODO: return Err if parsing fail
		let mut vdom = parse(&self.src, ParserOptions::default()).unwrap();

		let prop_regex = PROP_REGEX
			.get_or_init(|| Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap());

		for node in vdom.nodes_mut() {
			let bytes = match node {
				Node::Raw(r) => r,
				_ => continue,
			};

			let text = bytes.try_as_utf8_str().unwrap();

			let caps = prop_regex.captures(text).unwrap();

			let Some(prop) = caps.get(1) else { continue; };

			let Some(value) = props.get(prop.as_str()) else { continue; };

			bytes.set(value.as_str()).unwrap();
		}

		vdom.outer_html()
	}
}
