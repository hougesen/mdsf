use schemars::JsonSchema;

use crate::{
    config::default_enabled, formatters::sql_formatter::format_using_sql_formatter,
    formatters::sqlfluff::format_using_sqlfluff,
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum SqlFormatter {
    #[default]
    #[serde(rename = "sqlfluff")]
    Sqlfluff,
    #[serde(rename = "sql-formatter")]
    SQLFormatter,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod test_sql {
    use crate::{
        formatters::setup_snippet,
        languages::{sql::SqlFormatter, Language, LanguageFormatter},
    };

    use super::Sql;

    const INPUT: &str = "SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         ";

    const EXTENSION: &str = Language::Sql.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Sql::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Sql {
            enabled: false,
            formatter: SqlFormatter::SQLFormatter,
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());

        assert!(Sql {
            enabled: false,
            formatter: SqlFormatter::Sqlfluff,
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}
