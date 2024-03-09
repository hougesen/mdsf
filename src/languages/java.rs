use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::clang_format::format_using_clang_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum JavaFormatter {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Java {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: JavaFormatter,
}

impl Default for Java {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: JavaFormatter::default(),
        }
    }
}

impl LanguageFormatter for Java {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            JavaFormatter::ClangFormat => format_using_clang_format(snippet_path).map(|res| res.1),
        }
    }
}
