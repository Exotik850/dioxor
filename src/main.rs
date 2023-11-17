#![allow(non_snake_case, unused)]
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::rc::Rc;
use dioxus::html::input_data::keyboard_types::Key;
use log::LevelFilter;
use std::ops::Deref;
use log::info;
use dioxus::prelude::*;
use an_rope::Rope;
// use dioxus_fullstack::prelude::*;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Could not start logger!");
    // LaunchBuilder::new(app).launch();
    dioxus_web::launch(app);
}

#[derive(Debug, Clone, Copy)]
enum OpenedEditor {
    HTML,
    CSS,
    JS
}

#[derive(Debug, Clone, Default)]
struct EditorText {
    html: String,
    css: String,
    js: String,
}

impl EditorText {
    fn get_editor(&self, editor: OpenedEditor) -> &str {
        match editor {
            OpenedEditor::HTML => &self.html,
            OpenedEditor::CSS => &self.css,
            OpenedEditor::JS => &self.js,
        }
    }

    fn set_editor(&mut self, editor: OpenedEditor, text: String) {
        match editor {
            OpenedEditor::HTML => self.html = text,
            OpenedEditor::CSS => self.css = text,
            OpenedEditor::JS => self.js = text,
        }
    }

    fn add(&mut self, editor: OpenedEditor, key: &str) {
        match editor {
            OpenedEditor::HTML => self.html.push_str(key),
            OpenedEditor::CSS => self.css.push_str(key),
            OpenedEditor::JS => self.js.push_str(key),
        }
    }

    fn backspace(&mut self, editor: OpenedEditor) {
        match editor {
            OpenedEditor::HTML => self.html.pop(),
            OpenedEditor::CSS => self.css.pop(),
            OpenedEditor::JS => self.js.pop(),
        };
    }
}

fn app(cx: Scope) -> Element {

    let mut opened_editor = use_state(cx, || OpenedEditor::HTML);
    let mut editor_text = use_state(cx, EditorText::default);

    render!{
        div { class: "app",
            p { "Welcome to the editor!" }
            div { class: "tab-container",
                button { onclick: move |_| opened_editor.set(OpenedEditor::HTML), "HTML" }
                button { onclick: move |_| opened_editor.set(OpenedEditor::CSS), "CSS" }
                button { onclick: move |_| opened_editor.set(OpenedEditor::JS), "JS" }
            }
            br {}
            div { class: "editor-container",
                match *opened_editor.current() {
                    OpenedEditor::HTML => "HTML editor is open!",
                    OpenedEditor::CSS => "CSS editor is open!",
                    OpenedEditor::JS => "JS editor is open!",
                },
                textarea {
                    id: "editing",
                    value: "{editor_text.current().get_editor(**opened_editor)}",
                    style: "caret-color: black;",
                    spellcheck: "false",
                    // oninput: move |event| {
                    //     let value = event.data.value.clone();
                    //     editor_text.make_mut().set_editor(**opened_editor, value);
                    // }
                    onkeydown: move |evt| {
                        editor_text.with_mut(|f| {
                            match evt.key() {
                                Key::Character(c) => f.add(**opened_editor, &c),
                                Key::Tab => f.add(**opened_editor, "\t"),
                                Key::Enter => f.add(**opened_editor, "\n"),
                                Key::Backspace => f.backspace(**opened_editor),
                                _ => ()
                            }
                        })
                    }
                }
                pre { id: "highlighting",
                    editor_text.current().get_editor(**opened_editor).lines().chain(["\n"]).enumerate().map(|(i, line)| {
                        render! {
                            div {
                                display: "flex",
                                style: if i % 2 == 0 {
                                    "background: #4F4F4F"
                                } else {
                                    "background: #929292"
                                },
                                span { "{i+1} " }
                                code { "{line}" }
                            }
                        }
                    })
                }
            }
            iframe { id: "output", dangerous_inner_html: "{editor_text.current().get_editor(OpenedEditor::HTML)}" }
        }
    }
}

