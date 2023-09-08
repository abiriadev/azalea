use std::collections::HashMap;

use azalea;

#[test]
fn render() {
	let template = azalea::Template::new(r"<h1>{{title}}</h1>".to_owned());

	let props = {
		let mut h = HashMap::new();
		h.insert("title", "Hello, azalea!".to_owned());
		h
	};

	assert_eq!(r"<h1></h1>", template.render(&props));
}
