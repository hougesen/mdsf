use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::stylua::format_using_stylua};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum LuaFormatter {
    #[default]
    #[serde(rename = "stylua")]
    Stylua,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Lua {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: LuaFormatter,
}

impl Default for Lua {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: LuaFormatter::default(),
        }
    }
}

impl LanguageFormatter for Lua {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            LuaFormatter::Stylua => format_using_stylua(snippet_path),
        }
    }
}
