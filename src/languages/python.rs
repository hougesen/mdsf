use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{black::format_using_black, ruff::format_using_ruff},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum PythonFormatter {
    #[default]
    #[serde(rename = "ruff")]
    Ruff,
    #[serde(rename = "black")]
    Black,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Python {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: PythonFormatter,
}

impl Default for Python {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: PythonFormatter::default(),
        }
    }
}

impl LanguageFormatter for Python {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            PythonFormatter::Ruff => format_using_ruff(snippet_path).map(|res| res.1),
            PythonFormatter::Black => format_using_black(snippet_path).map(|res| res.1),
        }
    }
}
