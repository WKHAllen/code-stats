//! Code stats component.

use super::{Icon, LanguageStats, LanguageStatsTraversal};
use crate::icons::*;
use crate::services::*;
use dioxus::prelude::*;
use std::io;
use std::path::{Path, PathBuf};

/// The current state of the code statistics.
enum CodeStatsState {
    /// Currently fetching code statistics information.
    Fetching,
    /// Done fetching code statistics information.
    Complete(CodeStats),
    /// An error occurred while fetching code statistics information.
    Error(io::Error),
}

/// Code stats properties.
#[derive(Props)]
pub struct StatsProps<'a> {
    /// The path to display code statistics for.
    path: &'a Path,
    /// The callback to signal that the user is done viewing the statistics.
    on_done: EventHandler<'a, ()>,
}

/// Code stats component.
pub fn Stats<'a>(cx: Scope<'a, StatsProps<'a>>) -> Element {
    let status = use_state(cx, || CodeStatsState::Fetching);
    let subpath = use_state(cx, PathBuf::new);

    use_on_create(cx, || {
        let path = cx.props.path.to_owned();
        to_owned![status];
        async move {
            match collect_stats(path).await {
                Ok(stats) => status.set(CodeStatsState::Complete(stats)),
                Err(err) => status.set(CodeStatsState::Error(err)),
            }
        }
    });

    match &**status {
        CodeStatsState::Fetching => {
            render! {
                div {
                    class: "stats-fetching",
                    "Fetching code stats..."
                }
            }
        }
        CodeStatsState::Complete(stats) => {
            let substats = stats.stats_slice(&**subpath).unwrap();

            render! {
                div {
                    class: "stats-container",

                    div {
                        class: "stats",

                        div {
                            class: "stats-header",

                            div {
                                span {
                                    "Language breakdown for: "
                                }

                                div {
                                    class: "stats-path-container",

                                    span {
                                        class: "stats-path",
                                        cx.props.path.display().to_string()
                                    }
                                    span {
                                        class: "stats-subpath",
                                        "/"
                                        subpath.display().to_string()
                                    }
                                }
                            }

                            div {
                                button {
                                    r#type: "button",
                                    class: "icon-button",
                                    onclick: move |_| cx.props.on_done.call(()),

                                    Icon {
                                        data: XMARK
                                    }
                                }
                            }
                        }

                        LanguageStats {
                            label: "files",
                            stats: &substats.counts,
                            extractor: |counts| counts.files
                        }
                        LanguageStats {
                            label: "lines",
                            stats: &substats.counts,
                            extractor: |counts| counts.lines
                        }
                        LanguageStats {
                            label: "bytes",
                            stats: &substats.counts,
                            extractor: |counts| counts.bytes
                        }
                        LanguageStatsTraversal {
                            subpath: subpath,
                            dir_stats: substats,
                            on_traverse_down: move |dir| {
                                subpath.set(subpath.join(dir));
                            },
                            on_traverse_up: move |_| {
                                subpath.set(subpath.parent().unwrap_or(Path::new("")).to_path_buf());
                            }
                        }
                    }
                }
            }
        }
        CodeStatsState::Error(err) => {
            render! {
                div {
                    class: "error stats-error",
                    "An error occurred while fetching code stats: "
                    err.to_string()
                }
            }
        }
    }
}
