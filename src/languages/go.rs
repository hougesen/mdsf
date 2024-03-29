use schemars::JsonSchema;

use crate::formatters::{
    gofmt::format_using_gofmt, gofumpt::format_using_gofumpt, goimports::format_using_goimports,
    MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Go {
    #[default]
    #[serde(rename = "gofmt")]
    GoFmt,
    #[serde(rename = "gofumpt")]
    GoFumpt,
    #[serde(rename = "goimports")]
    GoImports,
}

impl Default for Lang<Go> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Go>::default(),
        }
    }
}

impl Default for MdsfFormatter<Go> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![
            Self::Single(Go::GoImports),
            Self::Multiple(vec![Self::Single(Go::GoFumpt), Self::Single(Go::GoFmt)]),
        ])
    }
}

impl LanguageFormatter for Go {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::GoFmt => format_using_gofmt(snippet_path),
            Self::GoFumpt => format_using_gofumpt(snippet_path),
            Self::GoImports => format_using_goimports(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_go {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Go;

    const INPUT: &str = "package main

   func add(a int , b int  ) int {
                return a + b
       }

    ";

    const EXTENSION: &str = crate::languages::Language::Go.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Go>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Go> {
            enabled: false,
            formatter: MdsfFormatter::<Go>::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(gofmt)]
    #[test]
    fn test_gofmt() {
        let l = Lang::<Go> {
            enabled: true,
            formatter: MdsfFormatter::Single(Go::GoFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "package main

func add(a int, b int) int {
\treturn a + b
}
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(gofumpt)]
    #[test]
    fn test_gofumpt() {
        let l = Lang::<Go> {
            enabled: true,
            formatter: MdsfFormatter::Single(Go::GoFumpt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "package main

func add(a int, b int) int {
\treturn a + b
}
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(goimports)]
    #[test]
    fn test_goimports() {
        let input = "package main

import (
\t\"os\"
\t\"fmt\"
)

func add(a int, b int) int {
\tfmt.Print(a)
\tfmt.Print(b)
\treturn a + b
}
";

        let expected_output = "package main

import (
\t\"fmt\"
)

func add(a int, b int) int {
\tfmt.Print(a)
\tfmt.Print(b)
\treturn a + b
}
";

        let l = Lang::<Go> {
            enabled: true,
            formatter: MdsfFormatter::Single(Go::GoImports),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
