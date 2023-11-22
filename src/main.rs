#![allow(non_snake_case, unused)]
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
mod file_explorer;

use editor::Editor;

use crate::file_explorer::FileExplorer;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Could not start logger!");
    // dioxus_web::launch(app);
    dioxus_desktop::launch_cfg(
        app,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()),
    )
}

#[inline_props]
fn ControlsBox<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(
        div { class: "w-1/3 border h-full bg-gray-600 z-2 m-1",
            children,
            button { class: "bg-blue-500 hover:bg-blue-700 text-white transition ease-in-out duration-150 hover:scale-110 font-bold py-2 px-4 rounded btn-trans",
                "Click me!"
            }
        }
    )
}

#[inline_props]
fn PanelsBox<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(
        div { class: "w-2/3 border h-full bg-gray-600 z-2 m-1", children }
    )
}

fn NavBar<'a>(cx: Scope<'a>) -> Element<'a> {
    render!( div { class: "flex w-full h-30 " } )
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
    use_shared_state_provider(cx, Config::default);
    let value = use_state(cx, String::new);

    render!(
        div { class: "w-screen h-screen bg-white-300 items-center justify-center overflow-hidden",
            div { class: "flex w-full h-full",
                ControlsBox { 
                    "Controls"
                    FileExplorer { base_dir: "C:\\Users\\KidKo\\Documents" }
                }
                PanelsBox { "Main Panels" }
            }
        }
    )
}
