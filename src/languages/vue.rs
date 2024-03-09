use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum VueFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Vue {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: VueFormatter,
}

impl Default for Vue {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: VueFormatter::default(),
        }
    }
}

impl LanguageFormatter for Vue {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            VueFormatter::Prettier => format_using_prettier(snippet_path, true).map(|res| res.1),
        }
    }
}
