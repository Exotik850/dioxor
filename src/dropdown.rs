use std::borrow::Borrow;

use dioxus::prelude::*;

#[derive(Props)]
pub struct DropdownItem<'a> {
    value: &'a str,
    callback: EventHandler<'a, MouseData>,
}

#[inline_props]
fn Dropdown<'a>(
    cx: Scope,
    items: &'a Vec<DropdownItem<'a>>,
    on_change: EventHandler<'a, String>,
) -> Element<'a> {
    let rendered_items = items
        .into_iter()
        .enumerate()
        .map(|(i, f)| render!( option { key: "{i}", value: f.value, "{f.value}" } ));

    render!(
        // select { onchange: move |evt| on_change.call(evt.borrow().data.value.as_str()), rendered_items }
        div {}
    )
}
