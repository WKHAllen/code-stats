//! A label for a programming language.

use crate::services::*;
use dioxus::prelude::*;

/// Language label properties.
#[derive(Props)]
pub struct LanguageLabelProps<'a> {
    /// The language.
    language: &'a str,
    /// The count for this language.
    count: Option<usize>,
    /// The total count across all languages.
    total: Option<usize>,
}

/// Programming language label component.
pub fn LanguageLabel<'a>(cx: Scope<'a, LanguageLabelProps<'a>>) -> Element {
    let (mut language_label, language_color) = get_language(cx.props.language);

    if let Some(count) = cx.props.count {
        language_label.push_str(": ");
        language_label.push_str(&format_with_commas(count));

        if let Some(total) = cx.props.total {
            language_label.push_str(&format!(
                " ({:.1}%)",
                (count as f64) / (total as f64) * 100.
            ));
        }
    }

    let language_color_style = format!("background-color: {};", language_color);

    render! {
        div {
            class: "lang-stats-lang",

            div {
                class: "lang-stats-lang-color",
                style: "{language_color_style}"
            }

            span {
                class: "lang-stats-lang-label",
                language_label
            }
        }
    }
}
