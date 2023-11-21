use dioxus::prelude::*;

#[derive(Props)]
pub struct DropdownItem<'a> {
	value: &'a str,
	callback: EventHandler<'a, MouseData>,
}


#[inline_props]
fn Dropdown<'a>(cx: Scope, items: &'a Vec<DropdownItem<'a>>) -> Element<'a> {
	let rendered_items = items.into_iter().enumerate().map(|(i, f)| {
		render!(
			option {
				key: "{i}",
				value: f.value,
				"{f.value}"
			}
		)
	});

	render!(
		select {
			rendered_items
		}
	)
}