use super::super::types::Color;
use std::io;

const OTHER_LANGUAGE_NAME: &'static str = "Other";
const OTHER_LANGUAGE_COLOR: &'static str = "#9f9f9f";

pub fn language_name_from_ext(extension: &str) -> Option<String> {
    let ext = if extension.starts_with(".") {
        extension[1..].to_lowercase().to_owned()
    } else {
        extension[..].to_lowercase().to_owned()
    };

    match match ext.as_str() {
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
    } {
        Some(value) => Some(value.to_owned()),
        None => None,
    }
}

pub fn language_color(language: &str) -> io::Result<Color> {
    match language {
        "HTML" => Color::from_html("#e34c26"),
        "CSS" => Color::from_html("#563d7c"),
        "SCSS" => Color::from_html("#c6538c"),
        "Sass" => Color::from_html("#a53b70"),
        "Less" => Color::from_html("#1d365d"),
        "JavaScript" => Color::from_html("#f1e05a"),
        "TypeScript" => Color::from_html("#2b7489"),
        "Vue" => Color::from_html("#41b883"),
        "Python" => Color::from_html("#3572a5"),
        "C" => Color::from_html("#555555"),
        "C++" => Color::from_html("#f34b7d"),
        "Objective-C" => Color::from_html("#438eff"),
        "C#" => Color::from_html("#178600"),
        "Java" => Color::from_html("#b07219"),
        "Go" => Color::from_html("#00add8"),
        "Rust" => Color::from_html("#dea584"),
        "Shell" => Color::from_html("#89e051"),
        "Batch" => Color::from_html("#c1f12e"),
        "PHP" => Color::from_html("#4f5d95"),
        "Assembly" => Color::from_html("#6e4c13"),
        "Lua" => Color::from_html("#000080"),
        "Nim" => Color::from_html("#ffc200"),
        "SQL" => Color::from_html("#e38c00"),
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "Unknown language")),
    }
}

pub fn get_lang(extension: &str) -> (String, Color) {
    match language_name_from_ext(extension) {
        Some(lang) => match language_color(&lang) {
            Ok(color) => (lang, color),
            Err(_) => (
                OTHER_LANGUAGE_NAME.to_owned(),
                Color::from_html(OTHER_LANGUAGE_COLOR).unwrap(),
            ),
        },
        None => (
            OTHER_LANGUAGE_NAME.to_owned(),
            Color::from_html(OTHER_LANGUAGE_COLOR).unwrap(),
        ),
    }
}

pub fn known_language(extension: &str) -> bool {
    language_name_from_ext(extension).is_some()
}
