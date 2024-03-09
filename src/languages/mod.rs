pub enum Language {
    Css,
    Dart,
    Elixir,
    Go,
    Gleam,
    Html,
    JavaScript,
    Json,
    Lua,
    Markdown,
    Nim,
    Python,
    Ruby,
    Rust,
    Shell,
    Toml,
    TypeScript,
    Vue,
    Yaml,
    Zig,
    // TODO: Cpp,
    // TODO: C,
    // TODO: Haskell,
    // TODO: OCaml,
    // TODO: Crystal,
    // TODO: PHP,
    // TODO: Java,
    // TODO: Kotlin,
    // TODO: CSharp,
    // TODO: FSharp,
    // TODO: Swift,
    // TODO: Svelte,
    // TODO: Julia,
    // TODO: Dockerfile,
    // TODO: XML,
    // TODO: D,
    // TODO: Erlang,
    // TODO: Elm,
    // TODO: Scala,
    // TODO: R,
    // TODO: GraphQL,
    // TODO: Protobuf,
    // TODO: SQL,
}

pub mod css;
pub mod dart;
pub mod elixir;
pub mod gleam;
pub mod go;
pub mod html;
pub mod javascript;
pub mod json;
pub mod lua;
pub mod markdown;
pub mod nim;
pub mod python;
pub mod ruby;
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
            "dart" => Some(Self::Dart),
            "elixir" => Some(Self::Elixir),
            "go" | "golang" => Some(Self::Go),
            "gleam" => Some(Self::Gleam),
            "html" => Some(Self::Html),
            "javascript" | "js" | "jsx" => Some(Self::JavaScript),
            "json" => Some(Self::Json),
            "lua" => Some(Self::Lua),
            "markdown" | "md" => Some(Self::Markdown),
            "nim" => Some(Self::Nim),
            "python" => Some(Self::Python),
            "ruby" => Some(Self::Ruby),
            "rust" | "rb" => Some(Self::Rust),
            "shell" | "sh" | "bash" | "zsh" => Some(Self::Shell),
            "toml" => Some(Self::Toml),
            "typescript" | "ts" | "tsx" => Some(Self::TypeScript),
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
            Self::Dart => ".dart",
            Self::Elixir => ".ex",
            Self::Go => ".go",
            Self::Gleam => ".gleam",
            Self::Html => ".html",
            Self::JavaScript => ".js",
            Self::Json => ".jsonc",
            Self::Lua => ".lua",
            Self::Markdown => ".md",
            Self::Nim => ".nim",
            Self::Python => ".py",
            Self::Ruby => ".rb",
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
