//! An icon component.

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
    let classes = match cx.props.class {
        Some(class) => format!("icon {}", class),
        None => "icon".to_owned(),
    };

    render! {
        div {
            class: "{classes}",
            dangerous_inner_html: cx.props.data
        }
    }
}
