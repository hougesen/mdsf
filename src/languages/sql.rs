use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::sql_formatter::format_using_sql_formatter};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum SQLFormatter {
    #[default]
    #[serde(rename = "sql-formatter")]
    SQLFormatter,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Sql {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: SQLFormatter,
}

impl Default for Sql {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: SQLFormatter::default(),
        }
    }
}

impl LanguageFormatter for Sql {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            SQLFormatter::SQLFormatter => format_using_sql_formatter(snippet_path).map(|res| res.1),
        }
    }
}
