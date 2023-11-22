use dioxus::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Props)]
pub struct FileExplorerProps<'a> {
    bind_filename: Option<&'a UseState<String>>,
    #[props(into)]
    base_dir: String,
}

struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    err: Option<String>,
}

impl Files {
    fn new(base_dir: String) -> Self {
        let mut files = Self {
            path_stack: vec![base_dir],
            path_names: vec![],
            err: None,
        };

        files.reload_path_list();

        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.last().unwrap();
        let paths = match std::fs::read_dir(cur_path) {
            Ok(e) => e,
            Err(err) => {
                let err = format!("An error occured: {err:?}");
                self.err = Some(err);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();

        // clear the current state
        self.clear_err();
        self.path_names.clear();

        for path in collected {
            self.path_names
                .push(path.unwrap().path().display().to_string());
        }
    }

    fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }

    fn current(&self) -> &str {
        self.path_stack.last().unwrap()
    }
    fn clear_err(&mut self) {
        self.err = None;
    }
}

pub fn FileExplorer<'a>(cx: Scope<'a, FileExplorerProps<'a>>) -> Element<'a> {
    let files = use_ref(cx, || Files::new(cx.props.base_dir.clone()));

    render!(
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
        header {
                i { class: "material-icons icon-menu", "menu" }
                h1 { "Files: ", files.read().current() }
                span { }
                i { class: "material-icons", onclick: move |_| files.write().go_up(), "logout" }
            }
           files.read().path_names.iter().enumerate().map(|(dir_id, path)| {
                    let path_end = path.split('/').last().unwrap_or(path.as_str());
                    let icon_type = if path_end.contains('.') {
                        "description"
                    } else {
                        "folder"
                    };
                    rsx! (
                        div {
                            class: "folder",
                            key: "{path}",
                            i { class: "material-icons",
                                onclick: move |_| files.write().enter_dir(dir_id),
                                "{icon_type}"
                                p { class: "cooltip", "0 folders / 0 files" }
                            }
                            h1 { "{path_end}" }
                        }
                    )
                }),
        files.read().err.as_ref().map(|err| {
                    rsx! (
                        div {
                            code { "{err}" }
                            button { onclick: move |_| files.write().clear_err(), "x" }
                        }
                    )
                })
    )
}
