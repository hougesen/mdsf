pub enum Language {
    Lua,
    Nim,
    Python,
    Rust,
    Zig,
}

impl Language {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "lua" => Some(Self::Lua),
            "nim" => Some(Self::Nim),
            "python" => Some(Self::Python),
            "rust" => Some(Self::Rust),
            "zig" => Some(Self::Zig),
            _ => None,
        }
    }
}
