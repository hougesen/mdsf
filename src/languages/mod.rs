pub enum Language {
    Html,
    JavaScript,
    Json,
    Lua,
    Markdown,
    Nim,
    Python,
    Rust,
    Toml,
    TypeScript,
    Zig,
}

pub mod html;
pub mod javascript;
pub mod json;
pub mod lua;
pub mod markdown;
pub mod nim;
pub mod python;
pub mod rust;
pub mod toml;
pub mod typescript;
pub mod zig;

pub trait LanguageFormatter {
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>>;
}

impl Language {
    pub fn maybe_from_str(input: &str) -> Option<Self> {
        match input {
            "html" => Some(Self::Html),
            "js" | "jsx" | "javascript" => Some(Self::JavaScript),
            "json" => Some(Self::Json),
            "lua" => Some(Self::Lua),
            "markdown" | "md" => Some(Self::Markdown),
            "nim" => Some(Self::Nim),
            "python" => Some(Self::Python),
            "rust" => Some(Self::Rust),
            "toml" => Some(Self::Toml),
            "ts" | "tsx" | "typescript" => Some(Self::TypeScript),
            "zig" => Some(Self::Zig),
            _ => None,
        }
    }

    pub const fn to_file_ext(&self) -> &'static str {
        match self {
            Self::Html => ".html",
            Self::JavaScript => ".js",
            Self::Json => ".jsonc",
            Self::Lua => ".lua",
            Self::Markdown => ".md",
            Self::Nim => ".nim",
            Self::Python => ".py",
            Self::Rust => ".rs",
            Self::Toml => ".toml",
            Self::TypeScript => ".ts",
            Self::Zig => ".zig",
        }
    }
}
