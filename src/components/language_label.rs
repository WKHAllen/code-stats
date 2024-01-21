//! A label for a programming language.

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
    render! {
        div {}
    }
}
