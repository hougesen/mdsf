use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{gofmt::format_using_gofmt, gofumpt::format_using_gofumpt},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum GoFormatter {
    #[default]
    #[serde(rename = "gofmt")]
    GoFmt,
    #[serde(rename = "gofumpt")]
    GoFumpt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Go {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: GoFormatter,
}

impl Default for Go {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: GoFormatter::default(),
        }
    }
}

impl LanguageFormatter for Go {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            GoFormatter::GoFmt => format_using_gofmt(snippet_path).map(|res| res.1),
            GoFormatter::GoFumpt => format_using_gofumpt(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Go, GoFormatter};

    const INPUT: &str = "package main

   func add(a int , b int  ) int {
                return a + b
       }

    ";

    const EXTENSION: &str = crate::languages::Language::Go.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Go::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Go {
            enabled: false,
            formatter: GoFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_gofmt() {
        let l = Go {
            enabled: true,
            formatter: GoFormatter::GoFmt,
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

    #[test]
    fn test_gofumpt() {
        let l = Go {
            enabled: true,
            formatter: GoFormatter::GoFumpt,
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
}
