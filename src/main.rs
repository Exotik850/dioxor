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

#[derive(Debug)]
struct Editor {
    title: String,
    body: String,
    cursor: usize,
}

#[derive(Debug)]
struct EditorText {
    pages: Vec<Editor>
}

impl Default for EditorText {
    fn default() -> Self {
        Self {
            pages: vec![
                Editor {
                    title: "HTML Editor".into(),
                    body: "".into(),
                    cursor: 0
                },
                Editor {
                    title: "CSS Editor".into(),
                    body: "".into(),
                    cursor: 0,
                },
                Editor {
                    title: "JS Editor".into(),
                    body: "".into(),
                    cursor: 0,
                }
            ]
        }
    }
}

impl EditorText {
    fn text(&self, editor: OpenedEditor) -> &Editor {
        match editor {
            OpenedEditor::HTML => &self.pages[0],
            OpenedEditor::CSS => &self.pages[1],
            OpenedEditor::JS => &self.pages[2],
        }
    }

    fn mut_text(&mut self, editor: OpenedEditor) -> &mut Editor {
        match editor {
            OpenedEditor::HTML => &mut self.pages[0],
            OpenedEditor::CSS => &mut self.pages[1],
            OpenedEditor::JS => &mut self.pages[2],
        }
    }

    fn add(&mut self, editor: OpenedEditor, key: &str) {
        self.mut_text(editor).body.push_str(key)
    }

    fn backspace(&mut self, editor: OpenedEditor) {
        self.mut_text(editor).body.pop();
    }

    fn handle_input(&mut self, editor: OpenedEditor, data: &KeyboardData) {
        let text_edit = self.mut_text(editor);
        match data.key() {
            Key::Character(ch) => self.add(editor, &ch),
            Key::Tab => self.add(editor, "\t"),
            Key::ArrowDown | Key::ArrowLeft | Key::ArrowUp | Key::ArrowRight => (),
            Key::Enter => self.add(editor, "\n"),
            Key::Backspace => self.backspace(editor),
            Key::Unidentified => panic!("Unidentified key!"),
            _ => ()

        }
    }
}

fn app(cx: Scope) -> Element {


    let mut opened_editor = use_state(cx, || OpenedEditor::HTML);
    let mut editor_text = use_ref(cx, EditorText::default);

    let textarea_element = use_state(cx, || None);

    let num_lines = editor_text.read().text(**opened_editor).body.lines().count();

    let binding = editor_text.read();
    let text = binding.text(**opened_editor);

    render!{
        div { class: "app",
            p { "Welcome to the editor!" }
            div { class: "tab-container",
                button { onclick: move |_| opened_editor.set(OpenedEditor::HTML), "HTML" }
                button { onclick: move |_| opened_editor.set(OpenedEditor::CSS), "CSS" }
                button { onclick: move |_| opened_editor.set(OpenedEditor::JS), "JS" }
            }
            div { class: "editor-container",
                textarea {
                    id: "editing",
                    value: "{text.body}",
                    style: "caret-color: black; height: {(num_lines + 1) * 15}px",
                    spellcheck: "false",
                    // rows: "{editor_text.current().text(**opened_editor).lines().count()}",
                    // oninput: move |event| {
                    //     let value = event.data.value.clone();
                    //     editor_text.make_mut().set_editor(**opened_editor, value);
                    // }
                    "selectionEnd": "",
                    prevent_default: "onkeydown",
                    onmounted: move |cx| textarea_element.set(Some(cx.inner().clone())),
                    onkeydown: move |evt| {
                        if matches!(&evt.data.key(), Key::ArrowDown | Key::ArrowLeft | Key::ArrowUp | Key::ArrowRight) {
                            if let Some(ele) = textarea_element.as_ref() {
                            }
                        } else {
                            editor_text.write().handle_input(**opened_editor, &evt.data)
                        }
                    },
                }
                pre { id: "highlighting",
                    text.body.lines().chain(["\n"]).enumerate().map(|(i, line)| {
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
            div { id: "output", dangerous_inner_html: "{text.body}", style: "{editor_text.read().text(OpenedEditor::CSS).body}" }
        }
    }
}

