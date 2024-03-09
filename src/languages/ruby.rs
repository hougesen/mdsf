use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::rubocop::format_using_rubocop};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum RubyFormatter {
    #[default]
    #[serde(rename = "rubocop")]
    RuboCop,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Ruby {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: RubyFormatter,
}

impl Default for Ruby {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: RubyFormatter::default(),
        }
    }
}

impl LanguageFormatter for Ruby {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            RubyFormatter::RuboCop => format_using_rubocop(snippet_path).map(|res| res.1),
        }
    }
}
