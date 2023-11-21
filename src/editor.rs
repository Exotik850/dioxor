
use dioxus::{prelude::*, html::textarea};

#[inline_props]
fn Editor<'a>(cx: Scope, on_change: EventHandler<'a, &'a FormData>, placeholder: &'a str) -> Element<'a> {

	render!(
		textarea {
			placeholder: *placeholder,
			onchange: move |evt| on_change.call(&evt.data),
			class: "editor"
		}
	)
}