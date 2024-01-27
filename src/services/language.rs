//! Programming language services.

/// The default name for unknown languages.
const OTHER_LANGUAGE_NAME: &str = "Other";

/// The default color for unknown languages.
const OTHER_LANGUAGE_COLOR: &str = "#9f9f9f";

/// A programming language.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Assembly,
    Batch,
    C,
    CPlusPlus,
    CSharp,
    Css,
    Go,
    Html,
    Java,
    JavaScript,
    Less,
    Lua,
    Nim,
    ObjectiveC,
    Php,
    Python,
    Rust,
    Sass,
    Scss,
    Shell,
    Sql,
    TypeScript,
    Vue,
    Unknown,
}

impl Language {
    /// Creates a new language instance given a file extension.
    pub fn new(ext: &str) -> Self {
        match ext {
            "asm" => Self::Assembly,
            "bat" | "cmd" => Self::Batch,
            "c" | "h" => Self::C,
            "cpp" | "cc" | "hpp" => Self::CPlusPlus,
            "cs" => Self::CSharp,
            "css" => Self::Css,
            "go" => Self::Go,
            "html" => Self::Html,
            "java" => Self::Java,
            "js" | "jsx" => Self::JavaScript,
            "less" => Self::Less,
            "lua" => Self::Lua,
            "nim" => Self::Nim,
            "m" => Self::ObjectiveC,
            "php" => Self::Php,
            "py" | "pyw" => Self::Python,
            "rs" => Self::Rust,
            "sass" => Self::Sass,
            "scss" => Self::Scss,
            "sh" | "bash" => Self::Shell,
            "sql" => Self::Sql,
            "ts" | "tsx" => Self::TypeScript,
            "vue" => Self::Vue,
            _ => Self::Unknown,
        }
    }

    /// Gets whether the language is known.
    pub fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Gets the name of the language.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Assembly => "Assembly",
            Self::Batch => "Batch",
            Self::C => "C",
            Self::CPlusPlus => "C++",
            Self::CSharp => "C#",
            Self::Css => "CSS",
            Self::Go => "Go",
            Self::Html => "HTML",
            Self::Java => "Java",
            Self::JavaScript => "JavaScript",
            Self::Less => "Less",
            Self::Lua => "Lua",
            Self::Nim => "Nim",
            Self::ObjectiveC => "Objective-C",
            Self::Php => "PHP",
            Self::Python => "Python",
            Self::Rust => "Rust",
            Self::Sass => "Sass",
            Self::Scss => "SCSS",
            Self::Shell => "Shell",
            Self::Sql => "SQL",
            Self::TypeScript => "TypeScript",
            Self::Vue => "Vue",
            Self::Unknown => OTHER_LANGUAGE_NAME,
        }
    }

    /// Gets the color of the language.
    pub fn color(&self) -> &'static str {
        match self {
            Self::Assembly => "#6e4c13",
            Self::Batch => "#c1f12e",
            Self::C => "#555555",
            Self::CPlusPlus => "#f34b7d",
            Self::CSharp => "#178600",
            Self::Css => "#563d7c",
            Self::Go => "#00add8",
            Self::Html => "#e34c26",
            Self::Java => "#b07219",
            Self::JavaScript => "#f1e05a",
            Self::Less => "#1d365d",
            Self::Lua => "#000080",
            Self::Nim => "#ffc200",
            Self::ObjectiveC => "#438eff",
            Self::Php => "#4f5d95",
            Self::Python => "#3572a5",
            Self::Rust => "#dea584",
            Self::Sass => "#a53b70",
            Self::Scss => "#c6538c",
            Self::Shell => "#89e051",
            Self::Sql => "#e38c00",
            Self::TypeScript => "#2b7489",
            Self::Vue => "#41b883",
            Self::Unknown => OTHER_LANGUAGE_COLOR,
        }
    }
}
