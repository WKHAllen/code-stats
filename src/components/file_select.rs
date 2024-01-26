//! File selection component.

use super::{Icon, Loading, PathDisplay};
use crate::icons::*;
use crate::services::*;
use dioxus::prelude::*;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};

/// The current state of the directory structure.
enum DirectoryInfoState {
    /// Currently fetching directory information.
    Fetching,
    /// Done fetching directory information.
    Completed(DirectoryInfo),
    /// An error occurred while fetching directory information.
    Error(io::Error),
}

/// File selection properties.
#[derive(Props)]
pub struct FileSelectProps<'a> {
    /// The path at which to start the selection.
    start_path: Option<&'a Path>,
    /// Whether the selection should allow directories instead of files.
    #[props(default = false)]
    directory: bool,
    /// Whether the selection should allow cancelation.
    #[props(default = false)]
    cancelable: bool,
    /// The path selection callback.
    on_select: EventHandler<'a, PathBuf>,
    /// The cancelation callback.
    on_cancel: Option<EventHandler<'a, ()>>,
}

/// File selection component.
pub fn FileSelect<'a>(cx: Scope<'a, FileSelectProps<'a>>) -> Element {
    let current_path = use_state(cx, || {
        cx.props.start_path.unwrap_or(Path::new("")).to_path_buf()
    });
    let selection = use_state(cx, || None);
    let selecting_directory = use_state(cx, || None::<PathBuf>);
    let status = use_state(cx, || DirectoryInfoState::Fetching);

    use_on_create(cx, {
        to_owned![current_path];
        || async move {
            if let Some(home_dir) = get_home_directory().await {
                current_path.set(home_dir);
            }
        }
    });

    let set_selection = |entry: &str, directory: bool| {
        let path = PathBuf::from(entry);

        if let DirectoryInfoState::Completed(_) = &**status {
            if cx.props.directory && directory {
                match &**selecting_directory {
                    Some(path) if path.to_str() == Some(entry) => {
                        let path = current_path.join(entry);
                        current_path.set(path.clone());
                        selection.set(Some(path));
                        selecting_directory.set(None);
                    }
                    _ => {
                        selection.set(Some(path.clone()));
                        selecting_directory.set(Some(path.clone()));
                    }
                }
            } else if !cx.props.directory && !directory {
                selection.set(Some(path));
                selecting_directory.set(None);
            } else if !cx.props.directory && directory {
                selection.set(None);
                selecting_directory.set(Some(path));
            }
        }
    };

    use_future(cx, (current_path,), |(current_path,)| {
        to_owned![status];
        async move {
            status.set(DirectoryInfoState::Fetching);

            match get_directory_info(&*current_path).await {
                Ok(dir_info) => status.set(DirectoryInfoState::Completed(dir_info)),
                Err(err) => status.set(DirectoryInfoState::Error(err)),
            }
        }
    });

    render! {
        div {
            class: "file-select",

            div {
                class: "file-select-header",

                div {
                    class: "file-select-header-title",

                    span {
                        "Select a "
                        if !cx.props.directory {
                            "file"
                        } else {
                            "directory"
                        }
                    }
                }

                div {
                    class: "file-select-header-path",

                    PathDisplay {
                        path: current_path
                    }

                    span {
                        dangerous_inner_html: "&#8203;"
                    }
                }

                div {
                    class: "file-select-header-actions",

                    button {
                        r#type: "button",
                        class: "icon-button",
                        disabled: **current_path == PathBuf::from("") || **current_path == PathBuf::from("/"),
                        onclick: move |_| {
                            if let Some(path) = current_path.parent() {
                                current_path.set(path.to_path_buf());
                                selection.set(Some(path.to_path_buf()));
                            } else {
                                current_path.set(PathBuf::from(""));
                                selection.set(None);
                            }
                        },

                        Icon {
                            data: ARROW_UP
                        }
                    }
                }
            }

            div {
                class: "file-select-body scrollbox",

                match &**status {
                    DirectoryInfoState::Fetching => render! {
                        Loading {
                            class: "dir-info-fetching",
                            text: "Fetching directory info..."
                        }
                    },
                    DirectoryInfoState::Completed(dir_info) => if !dir_info.dirs.is_empty() || !dir_info.files.is_empty() {
                        render! {
                            div {
                                class: "dir-info",

                                dir_info.dirs.iter().map(|entry| {
                                    let mut classes = vec!["dir-info-dir"];

                                    if let Some(path) = &**selection {
                                        if path.to_str() == Some(entry) {
                                            classes.push("dir-info-selected");
                                        }
                                    }

                                    if let Some(path) = &**selecting_directory {
                                        if path.to_str() == Some(entry) {
                                            classes.push("dir-info-selecting-directory");
                                        }
                                    }

                                    let classes = classes.join(" ");
                                    let entry_str = entry.strip_suffix(MAIN_SEPARATOR).unwrap_or(entry);

                                    render! {
                                        div {
                                            class: "{classes}",
                                            onclick: move |_| set_selection(entry.as_str(), true),

                                            Icon {
                                                data: FOLDER,
                                                class: "folder-icon"
                                            }
                                            span {
                                                entry_str
                                            }
                                        }
                                    }
                                })

                                dir_info.files.iter().map(|entry| {
                                    let mut classes = vec!["dir-info-file"];

                                    if let Some(path) = &**selection {
                                        if path.to_str() == Some(entry) {
                                            classes.push("dir-info-selected");
                                        }
                                    }

                                    let classes = classes.join(" ");

                                    render! {
                                        div {
                                            class: "{classes}",
                                            onclick: move |_| set_selection(entry.as_str(), false),

                                            Icon {
                                                data: FILE,
                                                class: "file-icon"
                                            }
                                            span {
                                                entry.as_str()
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    } else {
                        render! {
                            div {
                                class: "dir-info-info",
                                "Empty directory"
                            }
                        }
                    },
                    DirectoryInfoState::Error(err) => render! {
                        div {
                            class: "error dir-info-error",
                            "An error occurred while fetching directory info: "
                            {err.to_string()}
                        }
                    },
                }
            }

            div {
                class: "file-select-footer",

                div {
                    class: "file-select-selection-container",

                    match &**selection {
                        Some(path) => {
                            match path.iter()
                                .filter_map(|component|
                                    component.to_str()
                                        .filter(|&component| component != MAIN_SEPARATOR_STR))
                                .last()
                            {
                                Some(name) => render! {
                                    div {
                                        class: "file-select-selection",
                                        "Selected: "
                                        name
                                    }
                                },
                                None => render! {
                                    div {
                                        class: "file-select-selection",
                                        "No path selected"
                                    }
                                }
                            }
                        },
                        None => render! {
                            div {
                                class: "file-select-selection",
                                "No path selected"
                            }
                        },
                    }
                }

                div {
                    class: "file-select-actions-container",

                    div {
                        class: "file-select-actions",

                        if cx.props.cancelable {
                            render! {
                                button {
                                    r#type: "button",
                                    class: "button secondary",
                                    onclick: move |_| if let Some(on_cancel) = &cx.props.on_cancel {
                                        on_cancel.call(());
                                    },
                                    "Cancel"
                                }
                            }
                        }

                        button {
                            r#type: "button",
                            class: "button primary",
                            disabled: selection.is_none(),
                            onclick: move |_| cx.props.on_select.call(current_path.join(selection.as_ref().unwrap())),
                            "Select"
                        }
                    }
                }
            }
        }
    }
}
