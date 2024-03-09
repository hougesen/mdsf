use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::clang_format::format_using_clang_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum CppFormatter {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Cpp {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CppFormatter,
}

impl Default for Cpp {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CppFormatter::default(),
        }
    }
}

impl LanguageFormatter for Cpp {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CppFormatter::ClangFormat => format_using_clang_format(snippet_path).map(|res| res.1),
        }
    }
}
