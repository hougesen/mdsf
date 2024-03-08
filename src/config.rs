use schemars::JsonSchema;

use crate::languages::{
    javascript::JavaScript, json::Json, lua::Lua, nim::Nim, python::Python, rust::Rust, toml::Toml,
    typescript::TypeScript, zig::Zig,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct MdsfConfig {
    #[schemars(skip)]
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,
    #[serde(default)]
    pub javascript: JavaScript,
    #[serde(default)]
    pub json: Json,
    #[serde(default)]
    pub lua: Lua,
    #[serde(default)]
    pub nim: Nim,
    #[serde(default)]
    pub python: Python,
    #[serde(default)]
    pub rust: Rust,
    #[serde(default)]
    pub toml: Toml,
    #[serde(default)]
    pub typescript: TypeScript,
    #[serde(default)]
    pub zig: Zig,
}

impl Default for MdsfConfig {
    #[inline]
    fn default() -> Self {
        Self {
            schema: default_schema_location(),
            javascript: JavaScript::default(),
            json: Json::default(),
            lua: Lua::default(),
            nim: Nim::default(),
            python: Python::default(),
            rust: Rust::default(),
            toml: Toml::default(),
            typescript: TypeScript::default(),
            zig: Zig::default(),
        }
    }
}

#[inline]
pub const fn default_enabled() -> bool {
    true
}

#[inline]
fn default_schema_location() -> String {
    let package_version = env!("CARGO_PKG_VERSION");

    format!(
        "https://raw.githubusercontent.com/hougesen/mdsf/main/schemas/v{package_version}/mdsf.schema.json"
    )
}
