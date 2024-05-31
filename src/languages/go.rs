use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{
        crlfmt::format_using_crlfmt, gci::format_using_gci, gofmt::format_using_gofmt,
        gofumpt::format_using_gofumpt, goimports::format_using_goimports,
        goimports_reviser::format_using_goimports_reviser, golines::format_using_golines,
        MdsfFormatter,
    },
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Go {
    #[default]
    #[serde(rename = "gofmt")]
    GoFmt,
    #[serde(rename = "gofumpt")]
    GoFumpt,
    #[serde(rename = "goimports")]
    GoImports,
    #[serde(rename = "goimports-reviser")]
    GoImportsReviser,
    #[serde(rename = "crlfmt")]
    CrlFmt,
    #[serde(rename = "gci")]
    GCI,
    #[serde(rename = "golines")]
    GoLines,
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
            Self::Multiple(vec![
                Self::Single(Go::GCI),
                Self::Single(Go::GoImportsReviser),
                Self::Single(Go::GoImports),
            ]),
            Self::Multiple(vec![
                Self::Single(Go::GoFumpt),
                Self::Single(Go::GoFmt),
                Self::Single(Go::CrlFmt),
            ]),
        ])
    }
}

impl LanguageFormatter for Go {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::GoFmt => format_using_gofmt(snippet_path),
            Self::GoFumpt => format_using_gofumpt(snippet_path),
            Self::GoImports => format_using_goimports(snippet_path),
            Self::GoImportsReviser => format_using_goimports_reviser(snippet_path),
            Self::CrlFmt => format_using_crlfmt(snippet_path),
            Self::GCI => format_using_gci(snippet_path),
            Self::GoLines => format_using_golines(snippet_path),
        }
    }
}

impl core::fmt::Display for Go {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GoFmt => write!(f, "gofmt"),
            Self::GoFumpt => write!(f, "gofumpt"),
            Self::GoImports => write!(f, "goimports"),
            Self::GoImportsReviser => write!(f, "goimports-reviser"),
            Self::CrlFmt => write!(f, "crlfmt"),
            Self::GCI => write!(f, "gci"),
            Self::GoLines => write!(f, "golines"),
        }
    }
}

#[cfg(test)]
mod test_go {
    use super::Go;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

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
        .format(snippet_path, &LineInfo::fake())
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
            .format(snippet_path, &LineInfo::fake())
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
            .format(snippet_path, &LineInfo::fake())
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
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(golines)]
    #[test]
    fn test_golines() {
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
            formatter: MdsfFormatter::Single(Go::GoLines),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
