//! Loading spinner component.

use crate::classes::*;
use dioxus::prelude::*;

/// Loading spinner size.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LoadingSpinnerSize {
    /// A small spinner.
    Small,
    /// A medium size spinner.
    #[default]
    Medium,
    /// A large spinner.
    Large,
    /// A spinner that grows to the size of the container.
    Max,
}

impl LoadingSpinnerSize {
    /// Gets the name of the loading spinner size.
    pub fn size_name(&self) -> &'static str {
        match *self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Max => "max",
        }
    }
}

/// Loading spinner properties.
#[derive(Props)]
pub struct LoadingProps<'a> {
    /// Optional loading text.
    text: Option<&'a str>,
    /// The size of the spinner.
    #[props(default)]
    size: LoadingSpinnerSize,
    /// The CSS classes to apply.
    class: Option<&'a str>,
}

/// Loading spinner component.
pub fn Loading<'a>(cx: Scope<'a, LoadingProps<'a>>) -> Element {
    let container_class = classes!("loading-spinner-container", cx.props.class);
    let svg_class = classes!(
        "loading-spinner",
        format!("loading-spinner-{}", cx.props.size.size_name())
    );

    render! {
        div {
            class: "{container_class}",

            div {
                class: "loading-spinner-inner",

                if let Some(text) = cx.props.text {
                    render! {
                        span {
                            class: "loading-spinner-text",
                            text
                        }
                    }
                }

                svg {
                    class: "{svg_class}",
                    view_box: "0 0 50 50",

                    circle {
                        class: "spinner-path",
                        cx: 25,
                        cy: 25,
                        r: 20,
                        fill: "none",
                        stroke_width: 5
                    }
                }
            }
        }
    }
}
