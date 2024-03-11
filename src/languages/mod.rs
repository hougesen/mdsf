pub enum Language {
    C,
    Crystal,
    Cpp,
    CSharp,
    Css,
    Dart,
    Elixir,
    Gleam,
    Go,
    Html,
    Java,
    JavaScript,
    Json,
    Lua,
    Markdown,
    Nim,
    ObjectiveC,
    Protobuf,
    Python,
    Roc,
    Ruby,
    Rust,
    Shell,
    Sql,
    Toml,
    TypeScript,
    Vue,
    Yaml,
    Zig,
    // TODO: Haskell,
    // TODO: OCaml,
    // TODO: PHP,
    // TODO: Kotlin,
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
}

pub mod c;
pub mod cpp;
pub mod crystal;
pub mod csharp;
pub mod css;
pub mod dart;
pub mod elixir;
pub mod gleam;
pub mod go;
pub mod html;
pub mod java;
pub mod javascript;
pub mod json;
pub mod lua;
pub mod markdown;
pub mod nim;
pub mod objective_c;
pub mod protobuf;
pub mod python;
pub mod roc;
pub mod ruby;
pub mod rust;
pub mod shell;
pub mod sql;
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
            "c" | "clang" => Some(Self::C),
            "cpp" | "c++" => Some(Self::Cpp),
            "crystal" | "cr" => Some(Self::Crystal),
            "csharp" | "c#" => Some(Self::CSharp),
            "css" | "scss" => Some(Self::Css),
            "dart" => Some(Self::Dart),
            "elixir" => Some(Self::Elixir),
            "gleam" => Some(Self::Gleam),
            "go" | "golang" => Some(Self::Go),
            "html" => Some(Self::Html),
            "java" => Some(Self::Java),
            "javascript" | "js" | "jsx" => Some(Self::JavaScript),
            "json" => Some(Self::Json),
            "lua" => Some(Self::Lua),
            "markdown" | "md" => Some(Self::Markdown),
            "nim" => Some(Self::Nim),
            "objectivec" | "objective-c" | "objc" => Some(Self::ObjectiveC),
            "profobuf" | "profo" => Some(Self::Protobuf),
            "python" => Some(Self::Python),
            "roc" => Some(Self::Roc),
            "ruby" => Some(Self::Ruby),
            "rust" | "rb" => Some(Self::Rust),
            "shell" | "sh" | "bash" | "zsh" => Some(Self::Shell),
            "sql" | "bigquery" | "db2" | "db2i" | "hive" | "mariadb" | "mysql" | "n1ql"
            | "plsql" | "postgresql" | "redshift" | "singlestoredb" | "snowflake" | "spark"
            | "sqlite" | "transactsql" | "trino" | "tsql" => Some(Self::Sql),
            "toml" => Some(Self::Toml),
            "typescript" | "ts" | "tsx" => Some(Self::TypeScript),
            "vue" => Some(Self::Vue),
            "yml" | "yaml" => Some(Self::Yaml),
            "zig" => Some(Self::Zig),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_file_ext(&self) -> &'static str {
        match self {
            // NOTE: since scss is a superset of css we might as well support both at the same time
            Self::C => ".c",
            Self::Cpp => ".cpp",
            Self::Crystal => ".cr",
            Self::CSharp => ".cs",
            Self::Css => ".scss",
            Self::Dart => ".dart",
            Self::Elixir => ".ex",
            Self::Gleam => ".gleam",
            Self::Go => ".go",
            Self::Html => ".html",
            Self::Java => ".java",
            Self::JavaScript => ".js",
            Self::Json => ".jsonc",
            Self::Lua => ".lua",
            Self::Markdown => ".md",
            Self::Nim => ".nim",
            Self::ObjectiveC => ".m",
            Self::Protobuf => ".proto",
            Self::Python => ".py",
            Self::Roc => ".roc",
            Self::Ruby => ".rb",
            Self::Rust => ".rs",
            Self::Shell => ".sh",
            Self::Sql => ".sql",
            Self::Toml => ".toml",
            Self::TypeScript => ".ts",
            Self::Vue => ".vue",
            Self::Yaml => ".yml",
            Self::Zig => ".zig",
        }
    }
}
