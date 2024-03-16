use schemars::JsonSchema;

use crate::formatters::{
    sql_formatter::format_using_sql_formatter, sqlfluff::format_using_sqlfluff, MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Sql {
    #[default]
    #[serde(rename = "sqlfluff")]
    Sqlfluff,
    #[serde(rename = "sql-formatter")]
    SQLFormatter,
}

impl Default for Lang<Sql> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Sql>::default(),
        }
    }
}

impl Default for MdsfFormatter<Sql> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Sql::SQLFormatter),
            Self::Single(Sql::Sqlfluff),
        ])])
    }
}

impl LanguageFormatter for Sql {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::SQLFormatter => format_using_sql_formatter(snippet_path),
            Self::Sqlfluff => format_using_sqlfluff(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_sql {

    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Sql;

    const INPUT: &str = "SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         ";

    const EXTENSION: &str = crate::languages::Language::Sql.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Sql>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Sql> {
            enabled: false,
            formatter: MdsfFormatter::Single(Sql::SQLFormatter),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());

        assert!(Lang::<Sql> {
            enabled: false,
            formatter: MdsfFormatter::Single(Sql::Sqlfluff),
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

        let l = Lang::<Sql> {
            enabled: true,
            formatter: MdsfFormatter::Single(Sql::SQLFormatter),
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

        let l = Lang::<Sql> {
            enabled: true,
            formatter: MdsfFormatter::Single(Sql::Sqlfluff),
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
