//! A label for a programming language.

use crate::services::*;
use dioxus::prelude::*;

/// Language label properties.
#[derive(Props, PartialEq)]
pub struct LanguageLabelProps {
    /// The language.
    language: Language,
    /// The count for this language.
    count: Option<usize>,
    /// The total count across all languages.
    total: Option<usize>,
}

/// Programming language label component.
pub fn LanguageLabel(cx: Scope<LanguageLabelProps>) -> Element {
    let mut language_label = cx.props.language.name().to_owned();

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

    render! {
        div {
            class: "lang-stats-lang",

            div {
                class: "lang-stats-lang-color",
                background_color: cx.props.language.color()
            }

            span {
                class: "lang-stats-lang-label",
                language_label
            }
        }
    }
}
