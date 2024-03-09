use schemars::JsonSchema;

use crate::{
    config::default_enabled, formatters::sql_formatter::format_using_sql_formatter,
    formatters::sqlfluff::format_using_sqlfluff,
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum SqlFormatter {
    #[default]
    #[serde(rename = "sqlfluff")]
    Sqlfluff,
    #[serde(rename = "sql-formatter")]
    SQLFormatter,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Sql {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: SqlFormatter,
}

impl Default for Sql {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: SqlFormatter::default(),
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
            SqlFormatter::SQLFormatter => format_using_sql_formatter(snippet_path).map(|res| res.1),
            SqlFormatter::Sqlfluff => format_using_sqlfluff(snippet_path).map(|res| res.1),
        }
    }
}
