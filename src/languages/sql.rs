use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{
        format_multiple, sql_formatter::format_using_sql_formatter,
        sqlfluff::format_using_sqlfluff, MdsfFormatter,
    },
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum SqlFormatter {
    #[default]
    #[serde(rename = "sqlfluff")]
    Sqlfluff,
    #[serde(rename = "sql-formatter")]
    SQLFormatter,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Sql {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<SqlFormatter>,
}

impl Default for Sql {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<SqlFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<SqlFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![
            Self::Single(SqlFormatter::SQLFormatter),
            Self::Single(SqlFormatter::Sqlfluff),
        ])
    }
}

impl LanguageFormatter<SqlFormatter> for Sql {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        format_multiple(&self.formatter, snippet_path, &Self::format_single)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    fn format_single(
        formatter: &SqlFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            SqlFormatter::SQLFormatter => format_using_sql_formatter(snippet_path),
            SqlFormatter::Sqlfluff => format_using_sqlfluff(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_sql {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::{sql::SqlFormatter, Language, LanguageFormatter},
    };

    use super::Sql;

    const INPUT: &str = "SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         ";

    const EXTENSION: &str = Language::Sql.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Sql::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Sql {
            enabled: false,
            formatter: MdsfFormatter::Single(SqlFormatter::SQLFormatter),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());

        assert!(Sql {
            enabled: false,
            formatter: MdsfFormatter::Single(SqlFormatter::Sqlfluff),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_sql_formatter() {
        let expected_output = "SELECT
  *
FROM
  tbl
WHERE
  foo = 'bar';
";

        let l = Sql {
            enabled: true,
            formatter: MdsfFormatter::Single(SqlFormatter::SQLFormatter),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_sqlfluff() {
        let expected_output = "SELECT * FROM tbl
WHERE foo = 'bar';
";

        let l = Sql {
            enabled: true,
            formatter: MdsfFormatter::Single(SqlFormatter::Sqlfluff),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
