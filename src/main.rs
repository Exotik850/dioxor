#![allow(non_snake_case, unused)]
use an_rope::Rope;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use log::info;
use log::LevelFilter;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

mod dropdown;
mod editor;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Could not start logger!");
    dioxus_web::launch(app);
}

fn ControlsBox(cx: Scope) -> Element {
    render!(div {
        class: "ControlsBox"
    })
}

fn PanelsBox(cx: Scope) -> Element {
    render!(div { class: "PanelsBox" })
}

#[derive(Default, Debug)]
enum Language {
    #[default]
    None,
}

#[derive(Default, Debug)]
enum Theme {
    #[default]
    Dark,
    Light,
}

#[derive(Default, Debug)]
struct Config {
    lang: Language,
    theme: Theme,
}

fn app(cx: Scope) -> Element {
    let config = use_state(cx, Config::default);

    render!(
        div {
            class: "App",
            ControlsBox {}
            PanelsBox {}
            "{config:?}"
        }
    )
}
