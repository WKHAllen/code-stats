//! Traversal for viewing levels of code statistics.

use super::Icon;
use super::LanguageLabel;
use crate::icons::*;
use crate::services::*;
use dioxus::prelude::*;
use std::path::{Path, PathBuf};

/// Language statistics traversal properties.
#[derive(Props)]
pub struct LanguageStatsTraversalProps<'a> {
    /// The subpath within the traversal.
    subpath: &'a Path,
    /// The statistics at the current level of the traversal.
    dir_stats: &'a DirStats,
    /// The callback to trigger traversal down the directory structure.
    on_traverse_down: EventHandler<'a, &'a str>,
    /// The callback to trigger traversal up the directory structure.
    on_traverse_up: EventHandler<'a, ()>,
}

/// Language statistics traversal component.
pub fn LanguageStatsTraversal<'a>(cx: Scope<'a, LanguageStatsTraversalProps<'a>>) -> Element {
    let subpath_str = cx
        .props
        .subpath
        .iter()
        .map(|s| s.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");

    render! {
        div {
            class: "lang-stats-traversal",

            div {
                class: "lang-stats-traversal-header",

                span {
                    "/"
                    subpath_str
                }

                div {
                    button {
                        r#type: "button",
                        class: "icon-button",
                        disabled: cx.props.subpath == PathBuf::from("") || cx.props.subpath == PathBuf::from("/"),
                        onclick: move |_| cx.props.on_traverse_up.call(()),

                        Icon {
                            data: ARROW_UP
                        }
                    }
                }
            }

            div {
                class: "lang-stats-traversal-body scrollbox",

                if !cx.props.dir_stats.dirs.is_empty() || !cx.props.dir_stats.files.is_empty() {
                    render! {
                        div {
                            class: "lang-stats-traversal-dir-info",

                            cx.props.dir_stats.dirs.iter().map(|(name, stats)| {
                                let primary_language = stats.primary_language();

                                render! {
                                    div {
                                        class: "lang-stats-traversal-dir-info-dir",
                                        onclick: move |_| cx.props.on_traverse_down.call(name.as_str()),

                                        div {
                                            class: "lang-stats-traversal-dir-info-label",

                                            Icon {
                                                data: FOLDER,
                                                class: "folder-icon"
                                            }
                                            span {
                                                name.as_str()
                                            }
                                        }

                                        div {
                                            if let Some(language) = primary_language {
                                                render! {
                                                    LanguageLabel {
                                                        language: language
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            })

                            cx.props.dir_stats.files.keys().map(|name| {
                                let language = Language::new(PathBuf::from(&**name).extension().and_then(|s| s.to_str()).unwrap_or_default());

                                render! {
                                    div {
                                        class: "lang-stats-traversal-dir-info-file",

                                        div {
                                            class: "lang-stats-traversal-dir-info-label",

                                            Icon {
                                                data: FILE,
                                                class: "file-icon"
                                            }
                                            span {
                                                name.as_str()
                                            }
                                        }

                                        div {
                                            LanguageLabel {
                                                language: language
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                } else {
                    render! {
                        div {
                            class: "lang-stats-traversal-dir-info-info",
                            "Empty directory"
                        }
                    }
                }
            }
        }
    }
}
