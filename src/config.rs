use schemars::JsonSchema;

use crate::languages::{
    javascript::JavaScript, json::Json, lua::Lua, nim::Nim, python::Python, rust::Rust, toml::Toml,
    typescript::TypeScript, zig::Zig,
};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct MdsfConfig {
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

pub fn init_config_command() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;

    let conf = MdsfConfig::default();

    let config = serde_json::to_string_pretty(&conf)?;

    std::fs::write(current_dir.join("mdsf.json"), config)?;

    Ok(())
}

#[inline]
pub fn default_enabled() -> bool {
    true
}
