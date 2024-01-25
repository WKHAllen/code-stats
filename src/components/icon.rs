//! An icon component.

use crate::classes::*;
use dioxus::prelude::*;

/// Icon properties.
#[derive(Props)]
pub struct IconProps<'a> {
    /// The child element, expected here to be the SVG data.
    data: &'a str,
    /// The CSS classes to apply.
    class: Option<&'a str>,
}

/// Icon component.
pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = classes!("icon", cx.props.class);

    render! {
        div {
            class: "{class}",
            dangerous_inner_html: cx.props.data
        }
    }
}
