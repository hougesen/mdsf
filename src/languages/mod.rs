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
    Shell,
    Toml,
    TypeScript,
    Vue,
    Yaml,
    Zig,
    // TODO: Go,
    // TODO: Cpp,
    // TODO: C,
    // TODO: Haskell,
    // TODO: OCaml,
    // TODO: Crystal,
    // TODO: Ruby,
    // TODO: PHP,
    // TODO: Java,
    // TODO: Kotlin,
    // TODO: CSharp,
    // TODO: FSharp,
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
pub mod shell;
pub mod toml;
pub mod typescript;
pub mod vue;
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
            "sh" | "shell" | "bash" | "zh" => Some(Self::Shell),
            "toml" => Some(Self::Toml),
            "ts" | "tsx" | "typescript" => Some(Self::TypeScript),
            "yml" | "yaml" => Some(Self::Yaml),
            "zig" => Some(Self::Zig),
            "vue" => Some(Self::Vue),
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
            Self::Shell => ".sh",
            Self::Toml => ".toml",
            Self::TypeScript => ".ts",
            Self::Yaml => ".yml",
            Self::Zig => ".zig",
            Self::Vue => ".vue",
        }
    }
}
