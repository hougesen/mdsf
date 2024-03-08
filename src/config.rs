use schemars::JsonSchema;

use crate::languages::{
    javascript::JavaScript, json::Json, lua::Lua, nim::Nim, python::Python, rust::Rust, toml::Toml,
    typescript::TypeScript, zig::Zig,
};

#[derive(Debug, Default, serde::Deserialize, JsonSchema)]
pub struct MdsfConfig {
    pub json: Json,
    pub javascript: JavaScript,
    pub typescript: TypeScript,
    pub rust: Rust,
    pub lua: Lua,
    pub zig: Zig,
    pub toml: Toml,
    pub nim: Nim,
    pub python: Python,
}

#[inline]
pub fn default_enabled() -> bool {
    true
}
