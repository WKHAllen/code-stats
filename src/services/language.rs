//! Programming language services.

use std::fmt::Display;
use std::ops::Deref;

/// The default name for unknown languages.
const OTHER_LANGUAGE_NAME: &str = "Other";

/// The default color for unknown languages.
const OTHER_LANGUAGE_COLOR: &str = "#9f9f9f";

/// A programming language color.
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageColor(String);

impl LanguageColor {
    /// Creates a new language color.
    pub fn new<S>(inner: S) -> Self
    where
        S: Into<String>,
    {
        Self(inner.into())
    }
}

impl Deref for LanguageColor {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for LanguageColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Gets the name of a programming language from its file extension.
pub fn language_name_from_ext(extension: &str) -> Option<String> {
    let ext = extension
        .strip_prefix('.')
        .unwrap_or(extension)
        .to_lowercase();

    match ext.as_str() {
        "html" => Some("HTML"),
        "css" => Some("CSS"),
        "scss" => Some("SCSS"),
        "sass" => Some("Sass"),
        "less" => Some("Less"),
        "js" => Some("JavaScript"),
        "jsx" => Some("JavaScript"),
        "ts" => Some("TypeScript"),
        "tsx" => Some("TypeScript"),
        "vue" => Some("Vue"),
        "py" => Some("Python"),
        "pyw" => Some("Python"),
        "c" => Some("C"),
        "h" => Some("C"),
        "cpp" => Some("C++"),
        "cc" => Some("C++"),
        "hpp" => Some("C++"),
        "m" => Some("Objective-C"),
        "cs" => Some("C#"),
        "java" => Some("Java"),
        "go" => Some("Go"),
        "rs" => Some("Rust"),
        "sh" => Some("Shell"),
        "bash" => Some("Shell"),
        "bat" => Some("Batch"),
        "cmd" => Some("Batch"),
        "php" => Some("PHP"),
        "asm" => Some("Assembly"),
        "lua" => Some("Lua"),
        "nim" => Some("Nim"),
        "sql" => Some("SQL"),
        _ => None,
    }
    .map(ToOwned::to_owned)
}

/// Gets the color of a programming language.
pub fn language_color(language: &str) -> Option<LanguageColor> {
    match language {
        "HTML" => Some(LanguageColor::new("#e34c26")),
        "CSS" => Some(LanguageColor::new("#563d7c")),
        "SCSS" => Some(LanguageColor::new("#c6538c")),
        "Sass" => Some(LanguageColor::new("#a53b70")),
        "Less" => Some(LanguageColor::new("#1d365d")),
        "JavaScript" => Some(LanguageColor::new("#f1e05a")),
        "TypeScript" => Some(LanguageColor::new("#2b7489")),
        "Vue" => Some(LanguageColor::new("#41b883")),
        "Python" => Some(LanguageColor::new("#3572a5")),
        "C" => Some(LanguageColor::new("#555555")),
        "C++" => Some(LanguageColor::new("#f34b7d")),
        "Objective-C" => Some(LanguageColor::new("#438eff")),
        "C#" => Some(LanguageColor::new("#178600")),
        "Java" => Some(LanguageColor::new("#b07219")),
        "Go" => Some(LanguageColor::new("#00add8")),
        "Rust" => Some(LanguageColor::new("#dea584")),
        "Shell" => Some(LanguageColor::new("#89e051")),
        "Batch" => Some(LanguageColor::new("#c1f12e")),
        "PHP" => Some(LanguageColor::new("#4f5d95")),
        "Assembly" => Some(LanguageColor::new("#6e4c13")),
        "Lua" => Some(LanguageColor::new("#000080")),
        "Nim" => Some(LanguageColor::new("#ffc200")),
        "SQL" => Some(LanguageColor::new("#e38c00")),
        _ => None,
    }
}

/// Gets the name and color of a language.
pub fn get_language(extension: &str) -> (String, LanguageColor) {
    match language_name_from_ext(extension) {
        Some(lang) => match language_color(&lang) {
            Some(color) => (lang, color),
            None => (
                OTHER_LANGUAGE_NAME.to_owned(),
                LanguageColor::new(OTHER_LANGUAGE_COLOR),
            ),
        },
        None => (
            OTHER_LANGUAGE_NAME.to_owned(),
            LanguageColor::new(OTHER_LANGUAGE_COLOR),
        ),
    }
}

/// Returns whether the given language is known.
pub fn known_language(extension: &str) -> bool {
    language_name_from_ext(extension).is_some()
}
