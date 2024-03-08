pub enum Language {
    Css,
    Elixir,
    Gleam,
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
    Yaml,
    Zig,
}

pub mod css;
pub mod elixir;
pub mod gleam;
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
pub mod yaml;
pub mod zig;

pub trait LanguageFormatter {
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>>;
}

impl Language {
    #[inline]
    pub fn maybe_from_str(input: &str) -> Option<Self> {
        match input {
            "css" | "scss" => Some(Self::Css),
            "elixir" => Some(Self::Elixir),
            "gleam" => Some(Self::Gleam),
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
            "yml" | "yaml" => Some(Self::Yaml),
            "zig" => Some(Self::Zig),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_file_ext(&self) -> &'static str {
        match self {
            // NOTE: since scss is a superset of css we might as well support both at the same time
            Self::Css => ".scss",
            Self::Elixir => ".ex",
            Self::Gleam => ".gleam",
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
            Self::Yaml => ".yml",
            Self::Zig => ".zig",
        }
    }
}
