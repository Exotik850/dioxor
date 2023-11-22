use std::{borrow::Borrow, rc::Rc};

use dioxus::{html::textarea, prelude::*};

#[derive(Props)]
pub struct EditorProps<'a> {
    binding: &'a UseState<String>,
    #[props(default = "")]
    placeholder: &'a str,
}

pub fn Editor<'a>(cx: Scope<'a, EditorProps<'a>>) -> Element<'a> {
    let lines = cx.props.binding.lines().map(|f| render!( span { "{f}" } ));

    render!(
        div { class: "relative w-full h-full z-0",
            textarea {
                class: "absolute top-0 left-0 w-full h-full border-gray-400 rounded p-50 text-lg z-1",
                style: "resize: none",
                oninput: move |evt| cx.props.binding.set(evt.data.value.clone())
            }
            div { class: "absolute top-0 left-0 w-full h-full p-50 text-lg overflow-auto z-0 bg-transparent",
                lines
            }
        }
    )
}
