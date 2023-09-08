use std::{collections::HashMap, sync::OnceLock};

use regex::Regex;
use thiserror::Error;
use tl::{errors::SetBytesError, parse, Node, ParseError, ParserOptions};

static PROP_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct Template {
	src: String,
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("{0}")]
	ParseError(#[from] ParseError),

	#[error("{0}")]
	SetBytesError(#[from] SetBytesError),
}

impl Template {
	pub fn new(src: String) -> Self { Self { src } }

	pub fn render(
		&self,
		props: &HashMap<&'static str, String>,
	) -> Result<String, Error> {
		// TODO: return Err if parsing fail
		let mut vdom = parse(&self.src, ParserOptions::default())?;

		let prop_regex = PROP_REGEX
			.get_or_init(|| Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap());

		for node in vdom.nodes_mut() {
			let Node::Raw(bytes) = node else { continue; };

			let Some(text) = bytes.try_as_utf8_str() else { continue; };

			let Some(caps) = prop_regex.captures(text) else { continue; };

			let Some(prop) = caps.get(1) else { continue; };

			let Some(value) = props.get(prop.as_str()) else { continue; };

			bytes.set(value.as_str())?;
		}

		Ok(vdom.outer_html())
	}
}
