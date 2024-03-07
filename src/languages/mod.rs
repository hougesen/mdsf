pub enum Language {
    JavaScript,
    Json,
    Lua,
    Nim,
    Python,
    Rust,
    TypeScript,
    Zig,
}

impl Language {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "js" | "jsx" | "javascript" => Some(Self::JavaScript),
            "json" => Some(Self::Json),
            "lua" => Some(Self::Lua),
            "nim" => Some(Self::Nim),
            "python" => Some(Self::Python),
            "rust" => Some(Self::Rust),
            "ts" | "tsx" | "typescript" => Some(Self::TypeScript),
            "zig" => Some(Self::Zig),

            _ => None,
        }
    }

    pub fn to_file_ext(&self) -> &'static str {
        match self {
            Self::JavaScript => ".js",
            Self::Json => ".jsonc",
            Self::Lua => ".lua",
            Self::Nim => ".nim",
            Self::Python => ".py",
            Self::Rust => ".rs",
            Self::TypeScript => ".ts",
            Self::Zig => ".zig",
        }
    }
}
