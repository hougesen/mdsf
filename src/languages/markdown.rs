use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum MarkdownFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Markdown {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MarkdownFormatter,
}

impl Default for Markdown {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MarkdownFormatter::default(),
        }
    }
}

impl LanguageFormatter for Markdown {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            MarkdownFormatter::Prettier => format_using_prettier(snippet_path, false),
        }
    }
}
