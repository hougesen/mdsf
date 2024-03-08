pub enum Language {
    JavaScript,
    Json,
    Lua,
    Nim,
    Python,
    Rust,
    Toml,
    TypeScript,
    Zig,
}

pub mod javascript;
pub mod json;
pub mod lua;
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
            "js" | "jsx" | "javascript" => Some(Self::JavaScript),
            "json" => Some(Self::Json),
            "lua" => Some(Self::Lua),
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
            Self::JavaScript => ".js",
            Self::Json => ".jsonc",
            Self::Lua => ".lua",
            Self::Nim => ".nim",
            Self::Python => ".py",
            Self::Rust => ".rs",
            Self::Toml => ".toml",
            Self::TypeScript => ".ts",
            Self::Zig => ".zig",
        }
    }
}
