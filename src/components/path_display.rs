//! Path displaying component.

use dioxus::prelude::*;
use std::path::{Path, MAIN_SEPARATOR_STR};

/// Path properties.
#[derive(Props)]
pub struct PathDisplayProps<'a> {
    /// The path.
    path: &'a Path,
}

/// Path component.
pub fn PathDisplay<'a>(cx: Scope<'a, PathDisplayProps<'a>>) -> Element {
    let components = cx.props.path.iter().filter_map(|component| {
        component
            .to_str()
            .filter(|&component| component != MAIN_SEPARATOR_STR)
    });
    let num_components = components.clone().count();
    let components = components
        .flat_map(|component| {
            [
                render! {
                    div {
                        class: "path-component",

                        span {
                            component
                        }
                    }
                },
                render! {
                    div {
                        class: "path-separator",

                        span {
                            "/"
                        }
                    }
                },
            ]
        })
        .take((num_components * 2).saturating_sub(1));

    render! {
        div {
            class: "path",

            components
        }
    }
}
