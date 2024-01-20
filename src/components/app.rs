//! App component.

use super::{FileSelect, Stats};
use dioxus::prelude::*;
use std::path::PathBuf;

/// The state the application is in.
#[derive(Debug, Clone, Default)]
enum AppState {
    /// The home page.
    #[default]
    Home,
    /// The directory selection page.
    DirectorySelection,
    /// The code statistics page.
    DisplayingStats(PathBuf),
}

/// The top-level app component.
pub fn App(cx: Scope) -> Element {
    let app_state = use_state(cx, AppState::default);

    render! {
        div {
            class: "app",

            style {
                include_str!("../../assets/css/main.css")
            }

            match &**app_state {
                AppState::Home => render! {
                    div {
                        class: "home",

                        div {
                            class: "home-inner",

                            h1 {
                                class: "home-title",
                                "Code Statistics"
                            }
                            div {
                                class: "home-description",
                                "View detailed language statistics for any codebase."
                            }
                            button {
                                r#type: "button",
                                class: "button primary",
                                onclick: move |_| app_state.set(AppState::DirectorySelection),
                                "Get started"
                            }
                        }
                    }
                },
                AppState::DirectorySelection => render! {
                    div {
                        class: "path-select",

                        FileSelect {
                            directory: true,
                            cancelable: true,
                            on_select: move |path| app_state.set(AppState::DisplayingStats(path)),
                            on_cancel: move |_| app_state.set(AppState::Home)
                        }
                    }
                },
                AppState::DisplayingStats(path) => render! {
                    Stats {
                        path: path,
                        on_done: move |_| app_state.set(AppState::Home)
                    }
                }
            }
        }
    }
}
